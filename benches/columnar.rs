// Copyright 2017 columnar-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![feature(test)]

extern crate test;
extern crate column;
use column::Column;
use column::bitmap::FilteredCollection;
#[macro_use] extern crate column_derive;

use std::mem::size_of;

// The data for the benchmark, consiting of 8x64b=512b which is a cache line on most architectures
#[derive(Column, Debug, Default, Clone)]
struct Data {
    id: usize,
    val: f64,
    dummy: [usize; 6],
}

/// Perform assign operation on column type, input=output
#[bench]
fn data_column(b: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: 15., ..Data::default()});
    }
    let mut dc = <Data as Column>::with_capacity(size);
    dc.extend(a);
    b.bytes = (size_of::<usize>() * size) as u64;
    b.iter(|| {
        for e in dc.iter_mut() {
            *e.id *= 2;
        }
    })
}

/// Perform add/assign operation on row type with three inputs, one output
#[bench]
fn data_column_add_assign(bench: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    let mut b = Vec::with_capacity(size);
    let mut r = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: i as f64 * 0.6, ..Data::default()});
        b.push(Data { id: i + size, val: size as f64 + i as f64 * 0.6, ..Data::default()});
        r.push(Data { id: 0, val: 1., ..Data::default()});
    }
    let mut ca = <Data as Column>::with_capacity(size);
    let mut cb = <Data as Column>::with_capacity(size);
    let mut cr = <Data as Column>::with_capacity(size);
    ca.extend(a);
    cb.extend(b);
    test::black_box(r.first().unwrap().dummy);
    cr.extend(r);
    bench.bytes = (size_of::<f64>() * size * 3) as u64;
    bench.iter(|| {
        let zip: ::std::iter::Zip<_, _> = ca.iter().zip(cb.iter()).zip(cr.iter_mut());
        for ((ea, eb), er) in zip {
            *er.val += ea.val + eb.val;
        };
    })
}

/// Perform assign operation on row type, input=output
#[bench]
fn data_row_add_assign(bench: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    let mut b = Vec::with_capacity(size);
    let mut r = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: i as f64 * 0.6, ..Data::default()});
        b.push(Data { id: i + size, val: size as f64 + i as f64 * 0.6, ..Data::default()});
        r.push(Data { id: 0, val: 1., ..Data::default()});
    }
    test::black_box(r.first().unwrap().dummy);
    bench.bytes = (size_of::<f64>() * size * 3) as u64;
    bench.iter(|| {
        let zip: ::std::iter::Zip<_, _> = a.iter().zip(b.iter()).zip(r.iter_mut());
        for ((ea, eb), er) in zip {
            er.val += ea.val + eb.val;
        };
    })
}

/// Perform assign operation on row type, input=output
#[bench]
fn data_row(b: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: 15., ..Data::default()});
    }
    b.bytes = (size_of::<usize>() * size) as u64;
    b.iter(|| {
        for e in &mut a {
            e.id *= 2;
        }
    })
}

/// Perform add/assign operation on column type with three inputs, one output
#[bench]
fn data_bitmap_column_add_assign(bench: &mut test::Bencher) {
    let size = 1 << (20 + 1);
    let mut a = Vec::with_capacity(size);
    let mut b = Vec::with_capacity(size);
    let mut r = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: i as f64 * 0.6, ..Data::default()});
        b.push(Data { id: i + size, val: size as f64 + i as f64 * 0.6, ..Data::default()});
        r.push(Data { id: 0, val: 1., ..Data::default()});
    }
    let mut ca = <Data as Column>::with_capacity(size);
    let mut cb = <Data as Column>::with_capacity(size);
    let mut cr = <Data as Column>::with_capacity(size);
    ca.extend(a);
    cb.extend(b);
    test::black_box(r.first().unwrap().dummy);
    cr.extend(r);
    let mut bitmap_container: FilteredCollection<_> = FilteredCollection::new(&ca, ca.len());
    // Retain every second element
    bitmap_container.retain(|d| d.id & 1 == 1);
    // We touch three values but the bitmap only exposes every second element
    bench.bytes = (size_of::<f64>() * size * 3 / 2) as u64;
    bench.iter(|| {
        // iterate the filtered collection and zip it with `cb` and `cr`.
        let zip: ::std::iter::Zip<_, _> = bitmap_container.iter().zip(cb.iter()).zip(cr.iter_mut());
        for ((ea, eb), er) in zip {
            *er.val += ea.val + eb.val;
        };
    })
}

/// Perform add/assign operation on column type with three inputs, one output
#[bench]
fn data_bitmap_vec_add_assign(bench: &mut test::Bencher) {
    let size = 1 << (20 + 1);
    let mut a = Vec::with_capacity(size);
    let mut b = Vec::with_capacity(size);
    let mut r = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: i as f64 * 0.6, ..Data::default()});
        b.push(Data { id: i + size, val: size as f64 + i as f64 * 0.6, ..Data::default()});
        r.push(Data { id: 0, val: 1., ..Data::default()});
    }
    test::black_box(r.first().unwrap().dummy);
    let mut bitmap_container: FilteredCollection<_> = FilteredCollection::new(&a, a.len());
    // Retain every second element
    bitmap_container.retain(|d| d.id & 1 == 1);
    // We touch three values but the bitmap only exposes every second element
    bench.bytes = (size_of::<f64>() * size * 3 / 2) as u64;
    bench.iter(|| {
        // iterate the filtered collection and zip it with `b` and `r`.
        let zip: ::std::iter::Zip<_, _> = bitmap_container.iter().zip(b.iter()).zip(r.iter_mut());
        for ((ea, eb), er) in zip {
            er.val += ea.val + eb.val;
        };
    })
}

/// Perform assign operation on column type, input=output
#[bench]
fn data_row_to_column(b: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: 15., ..Data::default()});
    }
    b.bytes = (size_of::<Data>() * size) as u64;
    let mut dc = <Data as Column>::with_capacity(size);
    let a = &a;
    b.iter(move || {
        dc.clear();
        dc.extend(a.iter().cloned());
    })
}

