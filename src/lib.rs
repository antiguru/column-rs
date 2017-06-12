//! Columnar is a Rost library to repesent collections of elements
//! in a columnar memory layout.


#[cfg(test)] #[macro_use] extern crate columnar_derive;

/// Trait describing associated and generated types for a type
/// that can be represented in a columnar layout
pub trait Columnar<'a> {

    /// A reference type. It has the same fields as the original, only
    /// changed to references.
    type Ref;

    /// A mutable reference type. It has the same fields as the original,
    /// only changed to mutable references.
    type RefMut;

    /// The columnar container.
    type Columnar;

    /// An iterator over elements in the columnar container.
    type Iter;

    /// A mutable iterator over elements in the columnar container.
    type IterMut;
}

#[cfg(test)]
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
