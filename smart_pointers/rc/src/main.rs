fn main() {
    // use_box();
    use_rc();
}

// #[derive(Debug)]
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

#[derive(Debug)]
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

// use List::{Cons, Nil};
use std::rc::Rc;
use List2::{Cons, Nil};

// fn use_box() {
//     let a = Cons(5, 
//         Box::new(Cons(10,
//             Box::new(Nil))));
//     println!("a: {:?}", a);
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
//     println!("b: {:?}", b);
// }

fn use_rc() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a: {:?}", a);
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1

    let b = Cons(3, Rc::clone(&a));
    println!("b: {:?}", b);
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let c = Cons(4, Rc::clone(&a));
        println!("c: {:?}", c); // 3
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}