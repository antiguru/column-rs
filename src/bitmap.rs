//! Provide a filtered view on a collection.
//! # Examples
//! ```
//! use columnar::bitmap::FilteredCollection;
//! let collection = vec![1, 2, 3];
//! let mut filtered = FilteredCollection::new(&collection, collection.len());
//! filtered.retain(|&e| *e > 1);
//! ```

use std::iter::IntoIterator;

/// An iterator over a `FilteredCollection`
pub struct FilteredCollectionIterator<'a, I>
    where I: Iterator,
{
    /// An iterator over data elements
    iter_wrapped: I,
    /// A bitmap iterator to select visible elements
    iter_bitmap: ::std::slice::Iter<'a, bool>,
}

#[derive(Debug)]
/// A collection wrapper that can hide individual elements from iteration.
pub struct FilteredCollection<'a, A>
    where &'a A: IntoIterator,
          A: 'a,
{
    // Test ref or Rc
    /// The wrapped collection
    wrapped: &'a A,
    /// A bitmap indicting item visibility
    bitmap: Vec<bool>,
    /// The number of visible items
    // TODO: Should we really have this?
    valid_items: usize
}

impl<'a, A> FilteredCollection<'a, A>
    where &'a A: IntoIterator,
          A: 'a,
{

    /// Create a new `FilteredCollection`
    /// `new()` takes two arguments: a collection of type `A` to wrap and a lenght
    /// of the bitmap. The lenght should be less or equal to the number of elements
    /// in the collection. If the collection has more elements, only the first `len`
    /// elements will be visible.
    ///
    /// # Examples
    /// ```
    /// # use columnar::bitmap::FilteredCollection;
    /// let collection = vec![1, 2, 3];
    /// let mut filtered = FilteredCollection::new(&collection, collection.len());
    /// ```
    pub fn new(wrapped: &'a A, len: usize) -> Self {
        Self {
            wrapped,
            bitmap: vec![true; len],
            valid_items: len,
        }
    }

    /// Obtain an iterator on the visible elements in this `FilteredCollection`
    /// # Examples
    /// ```
    /// use columnar::bitmap::FilteredCollection;
    /// let collection : Vec<u64> = vec![1, 2, 3];
    /// let mut filtered = FilteredCollection::new(&collection, collection.len());
    /// filtered.retain(|&e| *e < 3);
    /// assert_eq!(filtered.iter().cloned().collect::<Vec<u64>>(), vec![1, 2]);
    /// ```
    pub fn iter(&'a self) -> FilteredCollectionIterator<'a, <&'a A as IntoIterator>::IntoIter> {
        FilteredCollectionIterator {
            iter_wrapped: self.wrapped.into_iter(),
            iter_bitmap: self.bitmap.iter(),
        }
    }

    pub fn iter_mut(&'a mut self) -> ::std::slice::Iter<'a, ()> {
        panic!("Not yet implemented")
    }

    /// The number of visible elements in this `FilteredCollection`
    ///
    /// Note that the result depends on the `len` parameter passed at construction time
    /// and might be different than the number of elements in the wrapped collection.
    ///
    /// # Examples
    /// ```
    /// use columnar::bitmap::FilteredCollection;
    /// let collection : Vec<u64> = vec![1, 2, 3];
    /// let mut filtered = FilteredCollection::new(&collection, collection.len());
    /// filtered.retain(|&e| *e < 3);
    /// assert_eq!(filtered.len(), 2);
    /// ```
    pub fn len(&'a self) -> usize {
        self.valid_items
    }

    /// Test if this `FilteredCollection` has any visibile elements
    /// # Examples
    /// ```
    /// use columnar::bitmap::FilteredCollection;
    /// let collection : Vec<u64> = vec![1, 2, 3];
    /// let mut filtered = FilteredCollection::new(&collection, collection.len());
    /// filtered.retain(|_| false);
    /// assert!(filtered.is_empty());
    /// # assert_eq!(filtered.len(), 0);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.valid_items == 0
    }

    /// Supply a function to select visible elements in this `FilteredCollection`
    /// # Examples
    /// ```
    /// use columnar::bitmap::FilteredCollection;
    /// let collection : Vec<u64> = vec![1, 2, 3];
    /// let mut filtered = FilteredCollection::new(&collection, collection.len());
    /// filtered.retain(|&e| *e == 2);
    /// # assert_eq!(filtered.iter().cloned().collect::<Vec<u64>>(), vec![2]);
    /// ```
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
                    return Some(item.expect("is_some wrong!"));
                }
            } else {
                // TODO: panicing here is bad because it might happen much later
                // than when the problem really happened, i.e. when creating
                // a FilteredCollection.
                // panic!("Bitmap iterator ended prematurely!");
                return None;
            }
            valid = self.iter_bitmap.next();
            item = self.iter_wrapped.next();
        }
        None
    }
}
