
library(classes)

p <- new("Animal")

stopifnot(mode(p) == "S4")

p$set_name("xyz")

stopifnot(p$name() == "xyz")

