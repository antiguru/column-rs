
#[macro_use] extern crate columnar_derive;
extern crate columnar;
use columnar::Columnar;

#[derive(Eq, PartialEq, Debug, Clone, Columnar)]
pub struct Useless {
    pub a: u64,
    b: Option<i64>,
}

#[test]
fn test() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}

#[test]
fn test_mul_2() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut original = u.clone();
    for e in &mut original {
        e.a *= 2;
    }
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    for e in &mut columnar {
        *e.a *= 2;
    }
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}


#[test]
fn test_index() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    for (index, e) in original.iter().enumerate() {
        assert_eq!(*e, columnar.index(index));
    }
}


#[test]
fn test_index_mut() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    for (index, e) in original.iter().enumerate() {
        *columnar.index_mut(index).a += 1;
        assert_eq!(e.a + 1, columnar.index(index).a);
    }
}

#[test]
fn test_clear() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let original = u.clone();
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    assert_eq!(columnar.len(), original.len());
    columnar.clear();
    assert_eq!(columnar.len(), 0);
}

#[test]
fn test_reserve() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.reserve(256);
    columnar.extend(u.into_iter());
    assert_eq!(columnar.capacity(), 256);
}

#[test]
fn test_is_empty() {
    let u = vec![Useless { a: 1, b: None}, Useless { a: 1, b: Some(-1)}];
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    assert!(columnar.is_empty());
    columnar.extend(u.into_iter());
    assert!(!columnar.is_empty());
}
