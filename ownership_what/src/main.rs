fn main() {
    demo_one();
    var_data_move();
    var_data_copy();
    stack_data_copy();

    let a = String::from("Hello, len");
    takes_ownership(a);
    // note: move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
    // println!("a: {}", a);

    let b = 8;
    makes_copy(b);
    println!("b: {}", b);

    let hi = gives_ownership();
    println!("Say: {}", hi);

    let _name = String::from("lencx");
    let nick_name = takes_and_gives_back(_name);
    println!("Nick Name: {}", nick_name);
    // note: move occurs because `_name` has type `std::string::String`, which does not implement the `Copy` trait
    // println!("Name: {}", _name);

    tuple_multi_val();
}


fn demo_one() {
    let s = "hello";
    {
        let s2 = "Len";
        println!("{}", s2);
    }
    println!("{}", s);

    let _s2 = String::from("Hi");
    let mut _s2 = String::from("Hello");
    _s2.push_str(", Len!!!");
    println!("{}", _s2);
    // println!("{}", s2);
}

fn var_data_move() {
    // number
    let x = 8;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // string
    let s1 = String::from("hello");
    let s2 = s1;
    // note: move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
    // println!("s1: {}", s1);
    println!("Move: s2: {}", s2);
}

fn var_data_copy() {
    let s1 = String::from("Hi");
    let s2 = s1.clone();
    println!("Clone: s1: {}, s2: {}", s1, s2);
}

fn stack_data_copy() {
    let x = 9;
    let y = x;
    println!("Stack Copy: x: {}, y: {}", x, y);
}


fn takes_ownership(some_str: String) {
    println!("String: {}", some_str);
}

fn makes_copy(some_num: u8) {
    println!("Number: {}", some_num);
}

fn gives_ownership() -> String {
    let some_str = String::from("Hi, Len");
    some_str
}

fn takes_and_gives_back(a_str: String) -> String {
    a_str
}

fn tuple_multi_val() {
    let s1 = String::from("hello, len!!");
    let (s2, len) = calc_len(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calc_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}