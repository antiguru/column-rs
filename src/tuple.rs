// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Provide a `Column` representation for tuples.

use ::Column;

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

            impl<'column, $($T),+> Col<($(Vec<$T>),+,)> {
                pub fn iter(&'column self) -> Col<($(::std::slice::Iter<'column, $T>),+,)> {
                    Col { t: ($(self.t.$idx.iter()),+,) }
                }
                pub fn iter_mut(&'column mut self) -> Col<($(::std::slice::IterMut<'column, $T>),+,)> {
                    Col { t: ($(self.t.$idx.iter_mut()),+,) }
                }
                pub fn len(&'column self) -> usize {
                    self.t.0.len()
                }
                pub fn is_empty(&'column self) -> bool {
                    self.t.0.is_empty()
                }
            }

            impl<'column, $($T),+> Column<'column> for Col<($($T),+,)> {
                type Output = Col<($(Vec<$T>),+,)>;
                fn new() -> Self::Output {
                    Col { t: ($(Vec::<$T>::new()),+,) }
                }
                fn with_capacity(capacity: usize) -> Self::Output {
                    Col { t: ($(Vec::<$T>::with_capacity(capacity)),+,) }
                }
            }

            impl<'column, $($T),+> Extend<($($T),+,)> for Col<($(Vec<$T>),+,)> {
                fn extend<T: IntoIterator<Item = ($($T),+,)>>(&mut self, iter: T) {
                    for element in iter {
                        ($(self.t.$idx.push(element.$idx)),+);
                    }
                }
            }
            impl<'column, $($T),+> IntoIterator for &'column Col<($(Vec<$T>),+,)> {
                type Item = ($(&'column $T),+,);
                type IntoIter = Col<($(::std::slice::Iter<'column, $T>),+,)>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter()
                }
            }
            impl<'column, $($T),+> Iterator for Col<($(::std::slice::Iter<'column, $T>),+,)> {
                type Item = ($(&'column $T),+,);
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
            impl<'column, $($T),+> IntoIterator for &'column mut Col<($(Vec<$T>),+,)> {
                type Item = ($(&'column mut $T),+,);
                type IntoIter = Col<($(::std::slice::IterMut<'column, $T>),+,)>;
                fn into_iter(self) -> Self::IntoIter {
                    self.iter_mut()
                }
            }
            impl<'column, $($T),+> Iterator for Col<($(::std::slice::IterMut<'column, $T>),+,)> {
                type Item = ($(&'column mut $T),+,);
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