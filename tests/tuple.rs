//! This test is disable because Tuples are not yet supported.

#[cfg(never)]

#[macro_use] extern crate columnar_derive;
extern crate columnar;

#[cfg(never)]
#[derive(Eq, PartialEq, Debug, Clone, Columnar)]
pub struct Useless(u64, Option<i64>);

#[cfg(never)]
#[test]
fn test() {
    let u = vec![Useless(1, None), Useless(1, Some(-1))];
    let original = u.clone();
    let mut columnar = UselessColumnar::with_capacity(u.len());
    columnar.extend(u.into_iter());
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}

#[cfg(never)]
#[test]
fn test_mul_2() {
    let u = vec![Useless(1, None), Useless(1, Some(-1))];
    let mut original = u.clone();
    for e in original.iter_mut() {
        e.0 *= 2;
    }
    let mut columnar = UselessColumnar::with_capacity(u.len());
    columnar.extend(u.into_iter());
    for e in columnar.iter_mut() {
        *e.0 *= 2;
    }
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}
