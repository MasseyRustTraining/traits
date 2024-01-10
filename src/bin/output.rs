pub trait TrG<T> {
    fn g(self) -> T;
}

impl TrG<u8> for i8 {
    fn g(self) -> u8 {
        self as u8 + 1
    }
}

impl TrG<u32> for i8 {
    fn g(self) -> u32 {
        self as u32 + 1
    }
}

pub trait TrA {
    type Output;

    fn a(self) -> Self::Output;
}

impl TrA for i8 {
    type Output = u32;

    fn a(self) -> u32 {
        self as u32 + 1
    }
}

fn main() {
    let x = 5.a();
    println!("{}", x);

    let x: u8 = 6.g();
    println!("{}", x);
}
