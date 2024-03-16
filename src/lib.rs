pub trait Doubleable : std::fmt::Debug {
    type Doubled;
    
    fn print(&self) {
        println!("{:?}", self);
    }

    fn double(&self) -> Self::Doubled;
}

impl Doubleable for u32 {
    type Doubled = u32;

    fn double(&self) -> Self::Doubled {
        self + self
    }
}

impl Doubleable for str {
    type Doubled = String;

    fn double(&self) -> Self::Doubled {
        let mut s = String::with_capacity(2 * self.len());
        s += self;
        s += self;
        s
    }
}
