// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! This test is disable because Tuples are not yet supported.

extern crate column;
use ::column::Column;
use ::column::tuple::Col;

#[test]
fn test() {
    let u = vec![(1, None), (1, Some(-1))];
    let original = u.clone();
    let mut column = <Col<(u64, Option<i32>)> as Column>::with_capacity(u.len());
    column.extend(u.into_iter());
    let result: Vec<_> = column.iter().map(|e| (*e.0, *e.1)).collect();
    assert_eq!(original, result);
}

