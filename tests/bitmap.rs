
#[macro_use] extern crate columnar_derive;
extern crate columnar;
use columnar::bitmap::*;
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
    let mut bitmap_container = FilteredCollection::new(&columnar, columnar.len());
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
    assert_eq!(columnar.len(), original.len());
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
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    for e in &mut columnar {
        *e.a *= 2;
    }
    let result: Vec<_> = columnar.iter().map(|e| UselessRef::to_owned(&e)).collect();
    assert_eq!(original, result);
}

#[test]
fn test_retain() {
    let size = 10;
    let mut u = vec![];
    for a in 0..size {
        u.push(Useless {a, b: None });
    }
    let mut columnar = <Useless as Columnar>::with_capacity(u.len());
    columnar.extend(u.into_iter());
    let mut bitmap_container = FilteredCollection::new(&columnar, columnar.len());
    bitmap_container.retain(|u| u.a & 1 == 0);
    println!("bitmap_container: {:?}", bitmap_container);
    assert_eq!(bitmap_container.len(), size as usize / 2);
    let as_vec: Vec<Useless> = bitmap_container.iter().map(|x| x.to_owned()).collect();
    assert_eq!(as_vec.len(), bitmap_container.len());
}