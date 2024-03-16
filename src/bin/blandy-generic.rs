//! Example from Blandy *et al*'s *Programming Rust*

trait Salad {
    fn name(&self) -> &'static str;
}

struct Lettuce;

impl Salad for Lettuce {
    fn name(&self) -> &'static str {
        "lettuce"
    }
}

struct Tomato;

impl Salad for Tomato {
    fn name(&self) -> &'static str {
        "tomato"
    }
}

fn show_salad<T: Salad>(salad: &[T]) {
    for s in salad {
        println!("{}", s.name());
    }
}

fn main() {
    let salad = [Lettuce, Lettuce];
    show_salad(&salad);
}
