# rust-analyzer-detecting-wrong-features
This repo shows that rust-analyzer does not raise an error that a module available on crate feature does not exist
when it is already included in other crate in the same workspace.

## Repo structure
1. Crate `stuff`. Inside there is a const `HELLO_UNCODITIONAL` and a module `feat` with another const `HELLO_CONDITIONAL`.
`stuff` provides a feature `feat` which, using `#[cfg(feature = "feat")]` conditional compilation,
allows access to the module `feat` and const `HELLO_CONDITIONAL` in it.
2. Crate `runnable`. Depends on crate `stuff` and its feature `feat`. Prints values of both constants in its main funciton.
3. Crate `runnable_no_feat`. Depends on crate `stuff` but not on its feature. Tries to also print values of both constants in its main funciton.

## Problem
Crate `runnable_no_feat` fails `cargo check` with an error "could not find `feat` in `stuff`", however `rust-analyzer` does not show any error in code.

If you remove dependency on feature `feat` from `runnable` and reload `rust-analyzer` then usages of `stuff::feat` in both `runnable` and `runnable_no_feat`
will be marked as erros by `rust-analyzer`. Module `feat` of crate `stuff` will also be greyed out. Now `runnable` crate also fails `cargo check`.

If you remove dependency on feature `feat` to runnable and reload `rust-analyzer` then no error will be seen and module `feat` will not be greyed out.
