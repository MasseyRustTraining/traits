use std::ops::Drop;

#[derive(Debug)]
struct MyStruct([u8; 2]);

impl Drop for MyStruct {
    fn drop(&mut self) {
        self.0[2] = 18;
        println!("dropping: {:?}", self.0);
    }
}

fn main() {
    let mut s = MyStruct([17; 2]);
    s.0[0] = 16;
    panic!("{:?}", s);
}
