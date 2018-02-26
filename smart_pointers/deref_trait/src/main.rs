fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
    assert_eq!(*z, 5);
    // *a == *(a.deref())
    assert_eq!(*a, 5);

    hello("Len"); // Hello, Len!!
    hello(&String::from("L")); // Hello, L!!
    let _rust = MyBox::new(String::from("Rust"));
    hello(&_rust); // Hello, Rust!!
    hello(&(*_rust)[..]); // Hello, Rust!!
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!!", name);
}