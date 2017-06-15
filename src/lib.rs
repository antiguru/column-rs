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

    /// The columnar container.
    type Container: Container<'a, Self>;

    /// An iterator over elements in the columnar container.
    type Iter: ::std::iter::Iterator;

    /// A mutable iterator over elements in the columnar container.
    type IterMut: ::std::iter::Iterator;

}

pub trait Container<'a, A: Columnar<'a>> {

    type Columnar: Columnar<'a>;

    fn iter(&'a self) -> A::Iter;
    // fn iter_mut(&'a mut self) -> A::IterMut;
    fn len(&'a self) -> usize;
}
