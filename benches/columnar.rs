#![feature(test)]

extern crate test;
extern crate columnar;
use columnar::Container;
use columnar::bitmap::ColumnarBitmapContainer;
#[macro_use] extern crate columnar_derive;

use std::mem::size_of;

#[derive(Columnar, Debug, Default)]
struct Data {
    id: usize,
    val: f64,
    dummy: [usize; 30],
}

#[bench]
fn data_columnar(b: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: 15., ..Data::default()});
    }
    let mut dc = DataContainer::with_capacity(size);
    dc.extend(a);
    b.bytes = (size_of::<usize>() * size) as u64;
    b.iter(|| {
        for mut e in dc.iter_mut() {
            *e.id *= 2;
        }
    })
}

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
    let mut ca = DataContainer::with_capacity(size);
    let mut cb = DataContainer::with_capacity(size);
    let mut cr = DataContainer::with_capacity(size);
    ca.extend(a);
    cb.extend(b);
    test::black_box(r.first().unwrap().dummy);
    cr.extend(r);
    bench.bytes = (size_of::<f64>() * size * 3) as u64;
    bench.iter(|| {
        let zip: ::std::iter::Zip<_, _> = ca.iter().zip(cb.iter()).zip(cr.iter_mut());
        for ((ea, eb), mut er) in zip {
            *er.val /= ea.val + eb.val;
        };
    })
}

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
        for ((ea, eb), mut er) in zip {
            er.val /= ea.val + eb.val;
        };
    })
}

#[bench]
fn data_row(b: &mut test::Bencher) {
    let size = 1 << 20;
    let mut a = Vec::with_capacity(size);
    for i in 0..size {
        a.push(Data { id: i, val: 15., ..Data::default()});
    }
    b.bytes = (size_of::<usize>() * size) as u64;
    b.iter(|| {
        for mut e in a.iter_mut() {
            e.id *= 2;
        }
    })
}

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
    let mut ca = DataContainer::with_capacity(size);
    let mut cb = DataContainer::with_capacity(size);
    let mut cr = DataContainer::with_capacity(size);
    ca.extend(a);
    cb.extend(b);
    test::black_box(r.first().unwrap().dummy);
    cr.extend(r);
    let mut bitmap_container: ColumnarBitmapContainer<_, _> = ColumnarBitmapContainer::new(&ca);
    bitmap_container.retain(|d| d.id & 1 == 1);
    bench.bytes = (size_of::<f64>() * size * 3 / 2) as u64;
    bench.iter(|| {
        let zip: ::std::iter::Zip<_, _> = bitmap_container.iter().zip(cb.iter()).zip(cr.iter_mut());
        for ((ea, eb), mut er) in zip {
            *er.val /= ea.val + eb.val;
        };
    })
}
