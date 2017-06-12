
#[macro_use] extern crate columnar_derive;
extern crate columnar;

#[derive(Eq, PartialEq, Debug, Clone, Columnar)]
pub struct Useless {
    pub a: u64,
    b: Option<i64>,
}

#[test]
fn test() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut columnar = UselessColumnar::with_capacity(u.len());
    columnar.extend(u.into_iter());
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}

#[test]
fn test_mul_2() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut original = u.clone();
    for e in original.iter_mut() {
        e.a *= 2;
    }
    let mut columnar = UselessColumnar::with_capacity(u.len());
    columnar.extend(u.into_iter());
    for e in columnar.iter_mut() {
        *e.a *= 2;
    }
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}
