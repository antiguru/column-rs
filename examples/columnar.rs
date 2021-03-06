// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate column;
#[macro_use] extern crate column_derive;

use column::Column;

#[derive(Column, Debug)]
struct Data {
    id: usize,
    val: f64,
}

#[derive(Column, Debug)]
pub struct DataGen<A: Copy> {
    id: A,
    val: f64,
}

fn main() {

    let mut u = <Data as Column>::new();

    let ds = vec![Data { id: 0, val: std::f64::consts::PI }, Data { id: 1, val: 42.}];
    u.extend(ds);

    for e in u.iter() {
        println!("Element: {:?}", e);
    }
    for e in u.iter_mut() {
        *e.val *= 2.;
    }
    for e in u.iter() {
        println!("Element: {:?}", e);
    }

    let mut g = <DataGen<&str> as Column>::new();
    g.extend(vec![DataGen {id: "A", val: 1.}]);
    for e in g.iter() {
        println!("Element: {:?}", e);
    }
}
