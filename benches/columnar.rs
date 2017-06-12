#![feature(test)]

extern crate test;
extern crate columnar;
#[macro_use] extern crate columnar_derive;

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
    let mut dc = DataColumnar::with_capacity(size);
    dc.extend(a);
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
    let mut ca = DataColumnar::with_capacity(size);
    let mut cb = DataColumnar::with_capacity(size);
    let mut cr = DataColumnar::with_capacity(size);
    ca.extend(a);
    cb.extend(b);
    test::black_box(r.first().unwrap().dummy);
    cr.extend(r);
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
    b.iter(|| {
        for mut e in a.iter_mut() {
            e.id *= 2;
        }
    })
}
