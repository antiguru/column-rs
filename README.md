# Columnar #

`columnar` is a Rust library to represent vectors of a certain type
in a columnar format. The columnar representation is beneficial when
iterating a large number of elements but only looking at a subset
of a type's fields.

# An example

To use `columnar`, add the following dependency to your project's
`Cargo.toml`:

```toml
[dependencies]
columnar = { git = "https://github.com/antiguru/columnar-rs.git" }
```

This will bring in the `columnar` crate from Github (this will hopefully change!),
which should allow you to use regular structs in a column-based memory layout.

```rust
extern crate columnar;
#[macro_use] extern crate columnar_derive;

#[derive(Columnar, Debug)]
struct Data {
    id: usize,
    val: f64,
}

fn main() {
    let mut u = <Data as Columnar>::new();

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

This example is contained in the `columnar` crate, you can run it from
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
#[derive(Columnar)]
struct DataGen<A: Copy> {
    id: A,
}
```

# Debugging

Columnar creates the required implementations during the compilation process. Sometimes things
go wrong and debugging the generated code is rather tedious. For this reason, a flag `verbose`
exists. It forces `columnar` to write the intermediate code to the target directory. For a type
`Data`, it will generate the file `target/derive_columnar_Data.rs`. (If there's a more elegant way,
please open an issue or send a pull request!) Insert the following snippet in `Cargo.toml` to
enable verbose output:

```toml
[dependencies]
columnar = { git = "https://github.com/antiguru/columnar-rs.git", features = [ "verbose" ] }
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
    Finished release [optimized + debuginfo] target(s) in 0.0 secs
[...]
running 4 tests
test data_columnar            ... bench:   1,696,509 ns/iter (+/- 29,321)
test data_columnar_add_assign ... bench:   5,868,536 ns/iter (+/- 244,826)
test data_row                 ... bench:  11,575,620 ns/iter (+/- 312,967)
test data_row_add_assign      ... bench:  25,229,786 ns/iter (+/- 932,355)

test result: ok. 0 passed; 0 failed; 0 ignored; 4 measured
```
