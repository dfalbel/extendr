#' Wrappers for Rust test functions.
#' 
#' Wrappers for Rust test functions.
#' @rdname wrappers
#' @export
hello_world <- function() {
  .Call(wrap__hello_world)
}

get_extendrtests_metadata <- function() .Call(wrap__get_extendrtests_metadata)
make_extendrtests_wrappers <- function(use_symbols=TRUE) .Call(wrap__make_extendrtests_wrappers, use_symbols)
