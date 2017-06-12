# Columnar #

`columnar` is a Rust library to represent vectors of a certain type
in a columnar format. The columnar representation is beneficial when
iterating a large number of elements but only looking at a subset
if a type's fields.

# An example

To use `columnar`, add the following dependency to your project's
`Cargo.toml`:

```
[dependencies]
columnar = { git = "https://github.com/antiguru/columnar-rs.git" }
```

This will bring in the `columnar` crate from Github (this will hopefully change!),
which should allow you to use regular structs in a column-based memory layout.

```
extern crate columnar;
#[macro_use] extern crate columnar_derive;

#[derive(Columnar, Debug)]
struct Data {
    id: usize,
    val: f64,
}

fn main() {
    let mut u = DataColumnar::new();

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
