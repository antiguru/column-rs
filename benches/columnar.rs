#![feature(test)]

extern crate test;
extern crate columnar;
use columnar::Columnar;
use columnar::bitmap::FilteredCollection;
#[macro_use] extern crate columnar_derive;

use std::mem::size_of;

// The data for the benchmark, consiting of 8x64b=512b which is a cache line on most architectures
#[derive(Columnar, Debug, Default)]
struct Data {
    id: usize,
    val: f64,
    dummy: [usize; 6],
}

/// Perform assign operation on columnar type, input=output
#[bench]
fn data_columnar(b: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: 15., ..Data::default()});
    }
    let mut dc = <Data as Columnar>::with_capacity(size);
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
fn data_columnar_add_assign(bench: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    let mut b = Vec::with_capacity(size);
    let mut r = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: i as f64 * 0.6, ..Data::default()});
        b.push(Data { id: i + size, val: size as f64 + i as f64 * 0.6, ..Data::default()});
        r.push(Data { id: 0, val: 1., ..Data::default()});
    }
    let mut ca = <Data as Columnar>::with_capacity(size);
    let mut cb = <Data as Columnar>::with_capacity(size);
    let mut cr = <Data as Columnar>::with_capacity(size);
    ca.extend(a);
    cb.extend(b);
    test::black_box(r.first().unwrap().dummy);
    cr.extend(r);
    bench.bytes = (size_of::<f64>() * size * 3) as u64;
    bench.iter(|| {
        let zip: ::std::iter::Zip<_, _> = ca.iter().zip(cb.iter()).zip(cr.iter_mut());
        for ((ea, eb), er) in zip {
            *er.val /= ea.val + eb.val;
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
            er.val /= ea.val + eb.val;
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

/// Perform add/assign operation on columnar type with three inputs, one output
#[bench]
fn data_bitmap_columnar_add_assign(bench: &mut test::Bencher) {
    let size = 1 << (20 + 1);
    let mut a = Vec::with_capacity(size);
    let mut b = Vec::with_capacity(size);
    let mut r = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: i as f64 * 0.6, ..Data::default()});
        b.push(Data { id: i + size, val: size as f64 + i as f64 * 0.6, ..Data::default()});
        r.push(Data { id: 0, val: 1., ..Data::default()});
    }
    let mut ca = <Data as Columnar>::with_capacity(size);
    let mut cb = <Data as Columnar>::with_capacity(size);
    let mut cr = <Data as Columnar>::with_capacity(size);
    ca.extend(a);
    cb.extend(b);
    test::black_box(r.first().unwrap().dummy);
    cr.extend(r);
    let mut bitmap_container: FilteredCollection<_> = FilteredCollection::new(&ca, ca.len());
    bitmap_container.retain(|d| d.id & 1 == 1);
    bench.bytes = (size_of::<f64>() * size * 3 / 2) as u64;
    bench.iter(|| {
        let zip: ::std::iter::Zip<_, _> = bitmap_container.iter().zip(cb.iter()).zip(cr.iter_mut());
        for ((ea, eb), er) in zip {
            *er.val /= ea.val + eb.val;
        };
    })
}

/// Perform add/assign operation on columnar type with three inputs, one output
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
    bitmap_container.retain(|d| d.id & 1 == 1);
    bench.bytes = (size_of::<f64>() * size * 3 / 2) as u64;
    bench.iter(|| {
        let zip: ::std::iter::Zip<_, _> = bitmap_container.iter().zip(b.iter()).zip(r.iter_mut());
        for ((ea, eb), er) in zip {
            er.val /= ea.val + eb.val;
        };
    })
}
