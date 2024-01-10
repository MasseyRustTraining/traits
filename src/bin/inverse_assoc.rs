trait Invertible {
    type Inverse;

    fn invert(self) -> Self::Inverse;
}

impl Invertible for i8 {
    type Inverse = i8;

    fn invert(self) -> Self::Inverse {
        -(self * 2)
    }
}

impl Invertible for i32 {
    type Inverse = i32;

    fn invert(self) -> Self::Inverse {
        -self
    }
}

impl Invertible for &str {
    type Inverse = String;

    fn invert(self) -> Self::Inverse {
        self.chars().rev().collect()
    }
}

impl Invertible for String {
    type Inverse = String;

    fn invert(self) -> Self::Inverse {
        self.chars().rev().collect()
    }
}

trait DoublyInvertible {
    type DoubleInverse;

    fn double_invert(self) -> Self::DoubleInverse;
}

impl DoublyInvertible for &str {
    type DoubleInverse = String;

    fn double_invert(self) -> Self::DoubleInverse {
        self.invert().invert()
    }
}

impl DoublyInvertible for i8 {
    type DoubleInverse = i8;

    fn double_invert(self) -> Self::DoubleInverse {
        self.invert().invert()
    }
}

impl DoublyInvertible for i32 {
    type DoubleInverse = i32;

    fn double_invert(self) -> Self::DoubleInverse {
        self.invert().invert()
    }
}

fn rev_twice<T: DoublyInvertible>(x: T) -> T::DoubleInverse {
    x.double_invert()
}

fn main() {
    println!("{}", rev_twice(17));
    println!("{}", rev_twice("hello"));
}
