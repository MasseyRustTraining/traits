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

fn show_salad(salad: &[Box<dyn Salad>]) {
    for s in salad {
        println!("{}", s.name());
    }
}

fn main() {
    let salad: [Box<dyn Salad>; 2] = [Box::new(Lettuce), Box::new(Tomato)];
    show_salad(&salad);
}
