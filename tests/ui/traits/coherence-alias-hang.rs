//@ check-pass
//@ revisions: current next
//[next]@ compile-flags: -Znext-solver

// Regression test for nalgebra hang <https://github.com/rust-lang/rust/issues/130056>.

#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type Id<T: ?Sized> = T;
trait NotImplemented {}

struct W<T: ?Sized, U: ?Sized>(*const T, *const U);
trait Trait {
    type Assoc: ?Sized;
}
impl<T: ?Sized + Trait> Trait for W<T, T> {
    type Assoc = W<T::Assoc, Id<T::Assoc>>;
}

trait Overlap<T: ?Sized> {}
impl<T: ?Sized> Overlap<T> for W<T, T> {}
impl<T: ?Sized + Trait + NotImplemented> Overlap<T::Assoc> for T {}

fn main() {}
