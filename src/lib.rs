//! Columnar is a Rust library to repesent collections of elements
//! in a columnar memory layout.


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
