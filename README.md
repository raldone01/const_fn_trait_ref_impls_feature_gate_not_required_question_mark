# Feature flag for `const_fn_trait_ref_impls` not required?!

Pull request: https://github.com/rust-lang/rust/pull/101802

## Reproduce
* `cargo +nightly-2022-09-16 run` - Observe that the code does not compile!
* `cargo +nightly-2022-09-17 run` - Observe the code works with the pull request merged but without using the newly added feature gate.
