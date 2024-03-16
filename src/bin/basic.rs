use traits::Doubleable;

#[derive(Debug)]
struct F32(f32);

impl Doubleable for F32 {
    type Doubled = f32;

    fn double(&self) -> f32 {
        self.0 + self.0
    }
}

fn double_thingy() -> impl Doubleable + std::fmt::Display {
    "hello"
}

fn quadruple<T>(x: T) -> <T as Doubleable>::Doubled
    where T: Doubleable
{
    x.print();
    x.print();
    x.double()
}

fn main() {
    println!("{}", 2u32.double());
    println!("{}", "hello".double());
    println!("{}", F32(2.0).double());
    println!("{}", quadruple(2u32));
    let x = double_thingy();
    x.print();
    println!("{}", x);
}
