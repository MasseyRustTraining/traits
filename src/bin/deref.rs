fn f(x: &str) {
    println!("{}", x);
}

fn g(a: &[u8]) {
    for v in a {
        println!("{}", v);
    }
}

#[allow(unused)]
fn _derefs() {
    let s: String = "hello".to_string();
    f(&s);
    let s: &str = &s;
    let a: [u8; 3] = [1, 2, 3];
    g(&a);
    let mut a: &[u8] = &a;
    let b: [u8; 2] = [1, 2];
    g(&b);
    a = &b;
    let v = vec![1, 2, 3, 4];
    a = &v;
}

fn main() {
    let a = &[1u8, 2, 3, 4];
    let na = a.len() - 1;
    println!("{:?}", &a[..na]);
}
