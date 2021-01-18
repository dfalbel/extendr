#' Wrappers for Rust test functions.
#' 
#' @rdname wrappers
#' @export
hello_world <- function() {
  .Call(wrap__hello_world)
}

#' Get module metadata as a list.
#' 
#' @rdname wrappers
#' @export
get_extendrtests_metadata <- function() .Call(wrap__get_extendrtests_metadata)

#' Build wrappers 
#' 
#' @param use_symbols use symbols instead of strings.
#' @rdname wrappers
#' @export
make_extendrtests_wrappers <- function(use_symbols=TRUE) .Call(wrap__make_extendrtests_wrappers, use_symbols)
