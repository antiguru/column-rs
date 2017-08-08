

use super::Columnar;

pub struct ColumnarBitmapIterator<'a, A: Columnar<'a>> {
    iter_wrapped: A::Iter,
    iter_bitmap: ::std::slice::Iter<'a, bool>,
}

#[derive(Debug)]
pub struct ColumnarBitmapContainer<'a, A: Columnar<'a>> where A: 'a {
    // Test ref or Rc
    wrapped: &'a A,
    bitmap: Vec<bool>,
    valid_items: usize
}

impl<'a, A> Columnar<'a> for ColumnarBitmapContainer<'a, A>
    where A: Columnar<'a>
{
    type Ref = A::Ref;
    type RefMut = A::RefMut;
    type Iter = ColumnarBitmapIterator<'a, A>;
    type IterMut = ::std::slice::Iter<'a, ()>;

    fn iter(&'a self) -> ColumnarBitmapIterator<'a, A> {
        ColumnarBitmapIterator {
            iter_wrapped: self.wrapped.iter(),
            iter_bitmap: self.bitmap.iter(),
        }
    }

    fn iter_mut(&'a mut self) -> Self::IterMut {
        panic!("Not yet implemented")
    }

    fn len(&'a self) -> usize {
        self.valid_items
    }
}

impl<'a, A> ColumnarBitmapContainer<'a, A>
    where A: Columnar<'a> + 'a
{
    pub fn new(wrapped: &'a A) -> Self {
        Self {
            wrapped,
            bitmap: vec![true; wrapped.len()],
            valid_items: wrapped.len(),
        }
    }

    pub fn retain<F>(&mut self, mut f: F) 
        where F: FnMut(&<<A as Columnar<'a>>::Iter as ::std::iter::Iterator>::Item) -> bool,
    {
        for (item, valid) in self.wrapped.iter().zip(self.bitmap.iter_mut()) {
            if *valid {
                *valid = f(&item);
                if !*valid {
                    self.valid_items -= 1;
                }
            }
        }
    }

}
// impl<'a, A, C> Extend<A> for ColumnarBitmapContainer<'a, A, C>
//     where C: Container<'a, A> + Extend<A> + 'a,
//           A: Columnar<'a> + 'a,
// {
//     fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
//         self.wrapped.extend(iter);
//     }
// }

impl<'a, A> IntoIterator for &'a ColumnarBitmapContainer<'a, A>
    where A: Columnar<'a> + 'a,
          A::Iter: ::std::iter::Iterator,
{
    type Item = <A::Iter as ::std::iter::Iterator>::Item;
    type IntoIter = ColumnarBitmapIterator<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A> Iterator for ColumnarBitmapIterator<'a, A>
    where A: Columnar<'a> + 'a,
          A::Iter: ::std::iter::Iterator,
{
    type Item = <A::Iter as ::std::iter::Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let mut valid = self.iter_bitmap.next();
        let mut item = self.iter_wrapped.next();
        while item.is_some() {
            if let Some(valid) = valid {
                if *valid {
                    return Some(item.expect("is_some wrong!"))
                }
            } else {
                panic!("Bitmap iterator ended prematurely!");
            }
            valid = self.iter_bitmap.next();
            item = self.iter_wrapped.next();
        }
        return None
    }
}

// impl<'columnar> IntoIterator for &'columnar mut DataColumnar {
//     type Item = DataRefMut<'columnar>;
//     type IntoIter = DataColumnarIteratorMut<'columnar>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_mut()
//     }
// }


// #[allow(dead_code)]
// pub struct DataColumnarIterator<'columnar> {
//     iter_id: ::std::slice::Iter<'columnar, usize>,
//     iter_val: ::std::slice::Iter<'columnar, f64>,
// }
// #[allow(dead_code)]
// pub struct DataColumnarIteratorMut<'columnar> {
//     iter_id: ::std::slice::IterMut<'columnar, usize>,
//     iter_val: ::std::slice::IterMut<'columnar, f64>,
// }
// impl DataColumnar {
//     pub fn new() -> Self {
//         DataColumnar {
//             id: Vec::new(),
//             val: Vec::new(),
//         }
//     }
//     pub fn with_capacity(capacity: usize) -> Self {
//         DataColumnar {
//             id: Vec::with_capacity(capacity),
//             val: Vec::with_capacity(capacity),
//         }
//     }
//     pub fn iter(&self) -> DataColumnarIterator {
//         DataColumnarIterator {
//             iter_id: self.id.iter(),
//             iter_val: self.val.iter(),
//         }
//     }
//     pub fn iter_mut(&mut self) -> DataColumnarIteratorMut {
//         DataColumnarIteratorMut {
//             iter_id: self.id.iter_mut(),
//             iter_val: self.val.iter_mut(),
//         }
//     }
// }
// impl<'columnar> Extend<Data> for DataColumnar {
//     fn extend<T: IntoIterator<Item = Data>>(&mut self, iter: T) {
//         for element in iter {
//             self.id.push(element.id);
//             self.val.push(element.val)
//         }
//     }
// }
// impl<'columnar> IntoIterator for &'columnar DataColumnar {
//     type Item = DataRef<'columnar>;
//     type IntoIter = DataColumnarIterator<'columnar>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }
// impl<'columnar> IntoIterator for &'columnar mut DataColumnar {
//     type Item = DataRefMut<'columnar>;
//     type IntoIter = DataColumnarIteratorMut<'columnar>;
//     fn into_iter(self) -> Self::IntoIter {
//         self.iter_mut()
//     }
// }
// #[allow(dead_code)]
// impl<'columnar> DataRef<'columnar> {
//     fn to_owned(&self) -> Data {
//         Data {
//             id: *self.id,
//             val: *self.val,
//         }
//     }
// }
// #[allow(dead_code)]
// impl<'columnar> DataRefMut<'columnar> {
//     fn to_owned(&self) -> Data {
//         Data {
//             id: *self.id,
//             val: *self.val,
//         }
//     }
// }
// impl<'columnar> Iterator for DataColumnarIterator<'columnar> {
//     type Item = DataRef<'columnar>;
//     fn next<'b>(&'b mut self) -> Option<Self::Item> {
//         let id = self.iter_id.next();
//         let val = self.iter_val.next();
//         if id.is_none() {
//             return None;
//         };
//         if val.is_none() {
//             return None;
//         }
//         let id = id.unwrap();
//         let val = val.unwrap();
//         Some(Self::Item { id, val })
//     }
// }
// impl<'columnar> Iterator for DataColumnarIteratorMut<'columnar> {
//     type Item = DataRefMut<'columnar>;
//     fn next<'b>(&'b mut self) -> Option<Self::Item> {
//         let id = self.iter_id.next();
//         let val = self.iter_val.next();
//         if id.is_none() {
//             return None;
//         };
//         if val.is_none() {
//             return None;
//         }
//         let mut id = id.unwrap();
//         let mut val = val.unwrap();
//         Some(Self::Item { id, val })
//     }
// }