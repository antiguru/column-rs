//! Provide a `Columnar` representation for tuples.

use ::Columnar;

/// A placeholder struct to wrap a type `T`. Here, `T` is used
/// to represent different kinds of tuples.
pub struct Col<T> {
    /// The wrapped data
    t: T,
}

// macro for implementing n-ary tuple functions and operations
#[doc(hidden)]
macro_rules! tuple_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(

            impl<'columnar, $($T),+> Col<($(Vec<$T>),+,)> {
                pub fn iter(&'columnar self) -> Col<($(::std::slice::Iter<'columnar, $T>),+,)> {
                    Col { t: ($(self.t.$idx.iter()),+,) }
                }
                pub fn iter_mut(&'columnar mut self) -> Col<($(::std::slice::IterMut<'columnar, $T>),+,)> {
                    Col { t: ($(self.t.$idx.iter_mut()),+,) }
                }
                pub fn len(&'columnar self) -> usize {
                    self.t.0.len()
                }
                pub fn is_empty(&'columnar self) -> bool {
                    self.t.0.is_empty()
                }
            }

            impl<'columnar, $($T),+> Columnar<'columnar> for Col<($($T),+,)> {
                type Output = Col<($(Vec<$T>),+,)>;
                fn new() -> Self::Output {
                    Col { t: ($(Vec::<$T>::new()),+,) }
                }
                fn with_capacity(capacity: usize) -> Self::Output {
                    Col { t: ($(Vec::<$T>::with_capacity(capacity)),+,) }
                }
            }

            impl<'columnar, $($T),+> Extend<($($T),+,)> for Col<($(Vec<$T>),+,)> {
                fn extend<T: IntoIterator<Item = ($($T),+,)>>(&mut self, iter: T) {
                    for element in iter {
                        ($(self.t.$idx.push(element.$idx)),+);
                    }
                }
            }
            impl<'columnar, $($T),+> IntoIterator for &'columnar Col<($(Vec<$T>),+,)> {
                type Item = ($(&'columnar $T),+,);
                type IntoIter = Col<($(::std::slice::Iter<'columnar, $T>),+,)>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }
            impl<'columnar, $($T),+> Iterator for Col<($(::std::slice::Iter<'columnar, $T>),+,)> {
                type Item = ($(&'columnar $T),+,);
                fn next(&mut self) -> Option<Self::Item> {
                    let t = ($(self.t.$idx.next()),+,);
                    $(
                    if t.$idx.is_none() {
                        return None;
                    }
                    )+
                    let t = ($(t.$idx.unwrap()),+,);
                    Some(t)
                }
            }
            impl<'columnar, $($T),+> IntoIterator for &'columnar mut Col<($(Vec<$T>),+,)> {
                type Item = ($(&'columnar mut $T),+,);
                type IntoIter = Col<($(::std::slice::IterMut<'columnar, $T>),+,)>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter_mut()
                }
            }
            impl<'columnar, $($T),+> Iterator for Col<($(::std::slice::IterMut<'columnar, $T>),+,)> {
                type Item = ($(&'columnar mut $T),+,);
                fn next(&mut self) -> Option<Self::Item> {
                    let t = ($(self.t.$idx.next()),+,);
                    $(
                    if t.$idx.is_none() {
                        return None;
                    }
                    )+
                    let t = ($(t.$idx.unwrap()),+,);
                    Some(t)
                }
            }
        )+
    }
}

tuple_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}