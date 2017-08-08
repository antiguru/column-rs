//! Columnar is a Rust library to repesent collections of elements
//! in a columnar memory layout.

pub mod bitmap;

/// Trait describing associated and generated types for a type
/// that can be represented in a columnar layout
pub trait Columnar<'a>: Sized {

    /// A reference type. It has the same fields as the original, only
    /// changed to references.
    type Ref;

    /// A mutable reference type. It has the same fields as the original,
    /// only changed to mutable references.
    type RefMut;

    /// An iterator over elements in the columnar container.
    type Iter: ::std::iter::Iterator;

    /// A mutable iterator over elements in the columnar container.
    type IterMut: ::std::iter::Iterator;

    fn iter(&'a self) -> Self::Iter;

    fn iter_mut(&'a mut self) -> Self::IterMut;

    fn len(&'a self) -> usize;
}

/// Trait to construct a `Columnar` collection for an output type. An implementation is generated.
/// # Example
///
/// ```
/// # #[macro_use] extern crate columnar_derive;
/// # extern crate columnar;
/// # use columnar::{Columnar, ColumnarFactory};
/// #[derive(Columnar)]
/// struct Data {x: usize}
/// # fn main() {
/// let columnar = <Data as ColumnarFactory>::new();
/// # }
/// ```
pub trait ColumnarFactory<'a> {

    /// The type representing the wrapped data in a columnar data layout.
    type Output: Columnar<'a>;

    /// Construct a new `Columar` with default capacity.
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate columnar_derive;
    /// # extern crate columnar;
    /// # use columnar::{Columnar, ColumnarFactory};
    /// #[derive(Columnar)]
    /// struct Data {x: usize}
    /// # fn main() {
    /// let columnar = <Data as ColumnarFactory>::new();
    /// # }
    /// ```
    fn new() -> Self::Output;

    /// Construct a new `Columar` with the provided capacity.
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate columnar_derive;
    /// # extern crate columnar;
    /// # use columnar::{Columnar, ColumnarFactory};
    /// #[derive(Columnar)]
    /// struct Data {x: usize}
    /// # fn main() {
    /// let columnar = <Data as ColumnarFactory>::with_capacity(200);
    /// # }
    /// ```
    fn with_capacity(len: usize) -> Self::Output;
}
