// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[rustc_attribute_should_be_reserved]
//~^ ERROR unless otherwise specified, attributes with the prefix `rustc_` are reserved
macro_rules! foo {
    () => (());
}

fn main() {
    foo!(); //~ ERROR cannot determine resolution for the macro `foo`
}
