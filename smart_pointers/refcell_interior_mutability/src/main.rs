fn main() {
    // println!("Hello, world!");
    // let x = 5;
    // cannot borrow immutable local variable `x` as mutable
    // let y = &mut x;
    let val = Rc::new(RefCell::new(3));
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(8)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(9)), Rc::clone(&a));

    *val.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;