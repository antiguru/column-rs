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

