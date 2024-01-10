trait Invertible<T> {
    fn invert(&self) -> T;
}

impl Invertible<i8> for i8 {
    fn invert(&self) -> Self {
        -(*self * 2)
    }
}

impl Invertible<i32> for i32 {
    fn invert(&self) -> Self {
        -*self
    }
}

impl Invertible<String> for str {
    fn invert(&self) -> String {
        self.chars().rev().collect()
    }
}

impl Invertible<String> for String {
    fn invert(&self) -> Self {
        self.chars().rev().collect()
    }
}

trait DoublyInvertible<T>
    where Self: Invertible<T>, T: Invertible<T>
{
    fn double_invert(&self) -> T {
        self.invert().invert()
    }
}

impl DoublyInvertible<String> for str {}
impl DoublyInvertible<i8> for i8 {}
impl DoublyInvertible<i32> for i32 {}

fn rev_twice<T: DoublyInvertible<U> + ?Sized, U: Invertible<U>>(x: &T) -> U {
    x.double_invert()
}

fn main() {
    println!("{}", rev_twice(&17));
    println!("{}", rev_twice("hello"));
}
