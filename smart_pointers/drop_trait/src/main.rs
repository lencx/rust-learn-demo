use std::mem::drop;

fn main() {
    let a = CustomSmartPointer {data: String::from("my stuff")};
    println!("CustomSmartPointers created.");
    // explicit destructor calls not allowed
    // a.drop();
    drop(a);
    println!("CustomSmartPointer drop before the end of main.");
    let b = CustomSmartPointer {data: String::from("other stuff")};
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}