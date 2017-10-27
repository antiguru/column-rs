// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use] extern crate column_derive;
extern crate column;
use column::Column;

#[derive(Eq, PartialEq, Debug, Clone, Column)]
pub struct Useless {
    pub a: u64,
    b: Option<i64>,
}

#[test]
fn test() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut column = <Useless as Column>::with_capacity(u.len());
    column.extend(u.into_iter());
    let result: Vec<_> = column.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}

#[test]
fn test_mul_2() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut original = u.clone();
    for e in &mut original {
        e.a *= 2;
    }
    let mut column = <Useless as Column>::with_capacity(u.len());
    column.extend(u.into_iter());
    for e in &mut column {
        *e.a *= 2;
    }
    let result: Vec<_> = column.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}


#[test]
fn test_index() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut column = <Useless as Column>::with_capacity(u.len());
    column.extend(u.into_iter());
    for (index, e) in original.iter().enumerate() {
        assert_eq!(*e, column.index(index));
    }
}


#[test]
fn test_index_mut() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut column = <Useless as Column>::with_capacity(u.len());
    column.extend(u.into_iter());
    for (index, e) in original.iter().enumerate() {
        *column.index_mut(index).a += 1;
        assert_eq!(e.a + 1, column.index(index).a);
    }
}

#[test]
fn test_clear() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut column = <Useless as Column>::with_capacity(u.len());
    column.extend(u.into_iter());
    assert_eq!(column.len(), original.len());
    column.clear();
    assert_eq!(column.len(), 0);
}

#[test]
fn test_reserve() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut column = <Useless as Column>::with_capacity(u.len());
    column.reserve(256);
    column.extend(u.into_iter());
    assert_eq!(column.capacity(), 256);
}

#[test]
fn test_is_empty() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut column = <Useless as Column>::with_capacity(u.len());
    assert!(column.is_empty());
    column.extend(u.into_iter());
    assert!(!column.is_empty());
}
