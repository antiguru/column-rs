// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! This test is disable because Tuples are not yet supported.

extern crate columnar;
use ::columnar::Columnar;
use ::columnar::tuple::Col;

#[test]
fn test() {
    let u = vec![(1, None), (1, Some(-1))];
    let original = u.clone();
    let mut columnar = <Col<(u64, Option<i32>)> as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    let result: Vec<_> = columnar.iter().map(|e| (*e.0, *e.1)).collect();
    assert_eq!(original, result);
}

