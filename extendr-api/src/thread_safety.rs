//! Provide limited protection for multithreaded access to the R API.

use std::sync::atomic::{AtomicU32, Ordering};

static OWNER_THREAD: AtomicU32 = AtomicU32::new(0);
static NEXT_THREAD_ID: AtomicU32 = AtomicU32::new(1);

thread_local! {
    static THREAD_ID: u32 = NEXT_THREAD_ID.fetch_add(1, Ordering::SeqCst);
}

// Get an integer 1.. for each thread that calls this.
pub fn this_thread_id() -> u32 {
    THREAD_ID.with(|&v| v)
}

/// Run a function single threaded.
/// Note: This will fail badly if the called function panics or calls RF_error.
///
/// ```
/// use extendr_api::*;
/// use std::sync::atomic::{AtomicU32, Ordering};
/// static GLOBAL_THREAD_COUNT: AtomicU32 = AtomicU32::new(0);
///
/// let threads : Vec<_> = (0..100).map(|_| {
///    std::thread::spawn(move|| {
///       single_threaded(|| {
///         // check that we are single threaded.
///         let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::AcqRel);
///         assert_eq!(old_thread_count, 0);
///         std::thread::sleep(std::time::Duration::from_millis(1));
///         GLOBAL_THREAD_COUNT.fetch_sub(1, Ordering::AcqRel);
///         // recursive calls are ok.
///         assert_eq!(single_threaded(|| {
///           1
///         }), 1);    
///       })
///    })
/// }).collect();
/// ```
pub fn single_threaded<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let id = this_thread_id();
    let old_id = OWNER_THREAD.load(Ordering::Acquire);

    if old_id != id {
        // wait for OWNER_THREAD to become 0 and put us as the owner.
        while OWNER_THREAD
            .compare_exchange(0, id, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    let res = f();

    if old_id != id {
        // release the lock and signal waiting threads.
        OWNER_THREAD.store(0, Ordering::Release);
    }

    res
}

/// This function is used by the wrapper logic to catch
/// panics on return.
pub fn handle_panic<F, R>(err_str: &str, f: F) -> R
where
    F: FnOnce() -> R,
    F: std::panic::UnwindSafe,
{
    match std::panic::catch_unwind(f) {
        Ok(res) => res,
        Err(_) => {
            unsafe {
                libR_sys::Rf_error(err_str.as_ptr() as *const std::os::raw::c_char);
            }
            unreachable!("handle_panic unreachable")
        }
    }
}
