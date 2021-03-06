# Column #

`column` is a Rust library to represent vectors of a certain type
in a columnar format. The columnar representation is beneficial when
iterating a large number of elements but only looking at a subset
of a type's fields.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# An example

To use `column`, add the following dependency to your project's
`Cargo.toml`:

```toml
[dependencies]
column = { git = "https://github.com/antiguru/column-rs.git" }
```

This will bring in the `column` crate from Github (this will hopefully change!),
which should allow you to use regular structs in a column-based memory layout.

```rust
extern crate column;
#[macro_use] extern crate column_derive;

#[derive(Column, Debug)]
struct Data {
    id: usize,
    val: f64,
}

fn main() {
    let mut u = <Data as Column>::new();

    let ds = vec![Data { id: 0, val: 3.141 }, Data { id: 1, val: 42.}];
    u.extend(ds);

    for e in u.iter() {
        println!("Element: {:?}", e);
    }
    for mut e in u.iter_mut() {
        *e.val *= 2.;
    }
    for e in u.iter() {
        println!("Element: {:?}", e);
    }

}
```

This example is contained in the `column` crate, you can run it from
the crate's root directory by typing

```
% cargo run --example columnar
Running `target/debug/examples/columnar`
Element: DataRef { id: 0, val: 3.141 }
Element: DataRef { id: 1, val: 42 }
Element: DataRef { id: 0, val: 6.282 }
Element: DataRef { id: 1, val: 84 }
```

# Generic types

The generated code dereferences values to materialize elements for `to_owned`.
That means that the value has to implement the `Copy` trait for this to work. Otherwise,
Rust will complain about it. For example, this will produce a working generic columnar
type:

```rust
#[derive(Column)]
struct DataGen<A: Copy> {
    id: A,
}
```

# Filtered collections

When using columnar types, they might be passed to different downstream functionality without
exposing all elements in the collection. To avoid intermediate copies, this carte contains
a filtered collection (`FilteredCollection`.) It is a wrapper around an `&IntoIterator`
combined with a `Vec<bool>` that stores which items are available in the target collection.

The following example instantiates a `FilteredCollection` and uses its `retain` method to only
retain a subset of elements in the collection. Note that this does not change the underlying data.
```rust
use column::bitmap::FilteredCollection;
let mut bitmap_container = FilteredCollection::new(&container, container.len());
bitmap_container.retain(|u| p(u));
```

# Debugging

Column creates the required implementations during the compilation process. Sometimes things
go wrong and debugging the generated code is rather tedious. For this reason, a flag `verbose`
exists. It forces `column` to write the intermediate code to the target directory. For a type
`Data`, it will generate the file `target/derive_column_Data.rs`. (If there's a more elegant way,
please open an issue or send a pull request!) Insert the following snippet in `Cargo.toml` to
enable verbose output:

```toml
[dependencies]
column = { git = "https://github.com/antiguru/column-rs.git", features = [ "verbose" ] }
```

The feature can also be activated on the command line when working on this project. The
following `cargo` invocation tests and dumps the intermediate files:

```
cargo test --features verbose
```

# Performance

There's a small benchmark. It shows that the columnar format can be substantially faster for
certain operations. Run it using Rust nightly using something like this:
```
% rustup run nightly cargo bench
[...]
running 6 tests
test data_bitmap_column_add_assign   ... bench:  13,178,990 ns/iter (+/- 405,043) = 1909 MB/s
test data_bitmap_vec_add_assign      ... bench:  39,891,687 ns/iter (+/- 714,537) = 630 MB/s
test data_column                     ... bench:   1,949,104 ns/iter (+/- 79,882) = 4303 MB/s
test data_column_add_assign          ... bench:   5,495,641 ns/iter (+/- 203,841) = 4579 MB/s
test data_row                        ... bench:  16,817,910 ns/iter (+/- 264,722) = 498 MB/s
test data_row_add_assign             ... bench:  32,581,004 ns/iter (+/- 759,734) = 772 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out
```

Take the performance numbers with a grain of salt. The speedup from using a columnar
representation originates from loading less and more dense data into memory. It will only
show any benefit of not all elements of a struct are accessed in a tight loop, because
only then we can reduce the number of bytes transferred into the CPU. However, when all
data is touched, i.e. all columns have to be loaded, the speedup may be negligible or even
negative. Also, there is a cost associated with transforming a row-based representation
into a column representation.

