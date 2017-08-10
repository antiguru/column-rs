//! Columnar is a Rust library to repesent collections of elements
//! in a columnar memory layout.

pub mod bitmap;
pub mod tuple;

/// Trait describing associated and generated types for a type
/// that can be represented in a columnar layout
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate columnar_derive;
/// # extern crate columnar;
/// # use columnar::Columnar;
/// #[derive(Columnar)]
/// struct Data {x: usize}
/// # fn main() {
/// let columnar = <Data as Columnar>::new();
/// # }
/// ```
pub trait Columnar<'a> {

    /// The type representing the wrapped data in a columnar data layout.
    type Output;

    /// Construct a new `Columar` with default capacity.
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate columnar_derive;
    /// # extern crate columnar;
    /// # use columnar::Columnar;
    /// #[derive(Columnar)]
    /// struct Data {x: usize}
    /// # fn main() {
    /// let columnar = <Data as Columnar>::new();
    /// # }
    /// ```
    fn new() -> Self::Output;

    /// Construct a new `Columar` with the provided capacity.
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate columnar_derive;
    /// # extern crate columnar;
    /// # use columnar::Columnar;
    /// #[derive(Columnar)]
    /// struct Data {x: usize}
    /// # fn main() {
    /// let columnar = <Data as Columnar>::with_capacity(200);
    /// # }
    /// ```
    fn with_capacity(len: usize) -> Self::Output;
}
