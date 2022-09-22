#![feature(const_trait_impl)]
// #![feature(const_fn_trait_ref_impls)] not required but should be
use core::marker::Destruct;

const fn test_fn() {}

const fn tester<T>(_fn: T)
where
  T: ~const Fn() + ~const Destruct,
{
}

const fn main() {
  tester(test_fn);
  let test_ref = &test_fn;
  tester(test_ref);
}
