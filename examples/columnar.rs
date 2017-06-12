extern crate columnar;
#[macro_use] extern crate columnar_derive;

#[derive(Columnar, Debug)]
struct Data {
    id: usize,
    val: f64,
}

#[derive(Columnar, Debug)]
struct DataGen<A: Copy> {
    id: A,
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

    let mut g = DataGenColumnar::new();
    g.extend(vec![DataGen {id: "A", val: 1.}]);
    for e in g.iter() {
        println!("Element: {:?}", e);
    }
}
