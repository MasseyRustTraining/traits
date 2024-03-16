use traits::Doubleable;

fn main() {
    let a: [&dyn Doubleable<Doubled = u32>; 2] = [
        &17,
        &23,
    ];
    for v in a {
        print!("{} ", v.double());
        v.print();
    }
}
