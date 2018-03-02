use std::rc::{Rc, Weak};
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    create_reference_cycle();
    create_tree_data();
    visualizing_changes();
}

fn create_reference_cycle() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count: {}", Rc::strong_count(&a)); // 1
    println!("a next item: {:?}", a.tail()); // a next item: Some(RefCell { value: Nil })

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation: {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count: {}", Rc::strong_count(&b)); // 1

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a: {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a: {}", Rc::strong_count(&a)); // 2

    // stack overflow
    // println!("a next item: {:?}", a.tail());
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn create_tree_data() {
    println!("-----------------------------");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf: {:?}", leaf);
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // None

    let branch = Rc::new(Node {
        value: 8,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("branch: {:?}", branch);
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
}

fn visualizing_changes() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 1, 0

    {
        let branch = Rc::new(Node {
            value: 9,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong: {}, weak: {}", Rc::strong_count(&branch), Rc::weak_count(&branch)); // 1, 1
        println!("leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 2, 0
    }
    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade()); // None
    println!("leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 1, 0

}