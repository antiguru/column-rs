// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[macro_use] extern crate column_derive;
extern crate column;
use column::bitmap::FilteredCollection;
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
    let mut bitmap_container = FilteredCollection::new(&column, column.len());
    let result: Vec<_> = column.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
    assert_eq!(column.len(), original.len());
    assert_eq!(bitmap_container.len(), original.len());

    for c in bitmap_container.iter() {
        assert_eq!(c.a, &1);
    }

    bitmap_container.retain(|item| item.b.is_none());

    assert_eq!(bitmap_container.len(), 1);
    for c in bitmap_container.iter() {
        assert_eq!(c.b, &None);
    }
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
fn test_retain() {
    let size = 10;
    let mut u = vec![];
    for a in 0..size {
        u.push(Useless {a, b: None });
    }
    let mut column = <Useless as Column>::with_capacity(u.len());
    column.extend(u.into_iter());
    let mut bitmap_container = FilteredCollection::new(&column, column.len());
    bitmap_container.retain(|u| u.a.trailing_zeros() > 0);
    println!("bitmap_container: {:?}", bitmap_container);
    assert_eq!(bitmap_container.len(), size as usize / 2);
    let as_vec: Vec<Useless> = bitmap_container.iter().map(|x| x.to_owned()).collect();
    assert_eq!(as_vec.len(), bitmap_container.len());
}
