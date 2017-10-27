// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Column is a Rust library to repesent collections of elements
//! in a columnar memory layout.

pub mod bitmap;
pub mod tuple;

/// Trait describing associated and generated types for a type
/// that can be represented in a columnar layout
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate column_derive;
/// # extern crate column;
/// # use column::Column;
/// #[derive(Column)]
/// struct Data {x: usize}
/// # fn main() {
/// let column = <Data as Column>::new();
/// # }
/// ```
pub trait Column<'a> {

    /// The type representing the wrapped data in a columnar data layout.
    type Output;

    /// Construct a new `Columar` with default capacity.
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate column_derive;
    /// # extern crate column;
    /// # use column::Column;
    /// #[derive(Column)]
    /// struct Data {x: usize}
    /// # fn main() {
    /// let column = <Data as Column>::new();
    /// # }
    /// ```
    fn new() -> Self::Output;

    /// Construct a new `Columar` with the provided capacity.
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate column_derive;
    /// # extern crate column;
    /// # use column::Column;
    /// #[derive(Column)]
    /// struct Data {x: usize}
    /// # fn main() {
    /// let column = <Data as Column>::with_capacity(200);
    /// # }
    /// ```
    fn with_capacity(len: usize) -> Self::Output;
}
