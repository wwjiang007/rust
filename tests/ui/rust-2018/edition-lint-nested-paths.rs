//@ edition: 2015
//@ run-rustfix

#![deny(absolute_paths_not_starting_with_crate)]

use foo::{a, b};
//~^ ERROR absolute paths must start with
//~| WARN this is accepted in the current edition
//~| ERROR absolute paths must start with
//~| WARN this is accepted in the current edition

mod foo {
    pub(crate) fn a() {}
    pub(crate) fn b() {}
    pub(crate) fn c() {}
}

fn main() {
    a();
    b();

    {
        use foo::{self as x, c};
        //~^ ERROR absolute paths must start with
        //~| WARN this is accepted in the current edition
        //~| ERROR absolute paths must start with
        //~| WARN this is accepted in the current edition
        x::a();
        c();
    }
}
