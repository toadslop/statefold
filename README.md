# Statefold

Statefold is a small library extending Rust's standard iterators with a standardized
way to write `fold` and `try_fold` operations in which state outside the result itself
needs to be maintained throughout each iteration in order to perform the operation.
The goal is to abstract away the common parts behind a common interface to streamline
writing such operations.
