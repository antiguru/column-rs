use std::iter::IntoIterator;

pub struct FilteredCollectionIterator<'a, I>
    where I: Iterator,
{
    iter_wrapped: I,
    iter_bitmap: ::std::slice::Iter<'a, bool>,
}

#[derive(Debug)]
pub struct FilteredCollection<'a, A>
    where &'a A: IntoIterator,
          A: 'a,
{
    // Test ref or Rc
    wrapped: &'a A,
    bitmap: Vec<bool>,
    valid_items: usize
}

impl<'a, A> FilteredCollection<'a, A>
    where &'a A: IntoIterator,
          A: 'a,
{

    pub fn new(wrapped: &'a A, len: usize) -> Self {
        Self {
            wrapped,
            bitmap: vec![true; len],
            valid_items: len,
        }
    }

    pub fn iter(&'a self) -> FilteredCollectionIterator<'a, <&'a A as IntoIterator>::IntoIter> {
        FilteredCollectionIterator {
            iter_wrapped: self.wrapped.into_iter(),
            iter_bitmap: self.bitmap.iter(),
        }
    }

    pub fn iter_mut(&'a mut self) -> ::std::slice::Iter<'a, ()> {
        panic!("Not yet implemented")
    }

    pub fn len(&'a self) -> usize {
        self.valid_items
    }

    pub fn retain<F>(&mut self, mut f: F) 
        where F: FnMut(&<&'a A as IntoIterator>::Item) -> bool,
    {
        for (valid, item) in self.bitmap.iter_mut().zip(self.wrapped.into_iter()) {
            if *valid {
                *valid = f(&item);
                if !*valid {
                    self.valid_items -= 1;
                }
            }
        }
    }

}

impl<'a, I> Iterator for FilteredCollectionIterator<'a, I>
    where I: Iterator,
{
    type Item = I::Item;
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
        None
    }
}
