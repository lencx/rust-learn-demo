fn main() {
    println!("Hello, world!");
    another_fn();
    parameter_fn(2356);
    parameters_fn(19, 'a');
    a();

    sum(17);

    let _plus_one = plus_one(9);
    println!("plus_one: {}", _plus_one);
}

fn another_fn() {
    println!("Another function.");
}

fn parameter_fn(x: u16) {
    println!("The value of x is: {}", x);
}

fn parameters_fn(x: u16, y: char) {
    println!("parameters_fn: x is: {}, y is: {}", x, y);
}

fn a() {
    let x = 5;
    let y = {
        let x = 7;
        x + 1
    };
    println!("x is: {}, y is: {}", x, y);
}

fn sum(x: i32) {
    let _sum = x + num();
    println!("{}", _sum);
}
fn num() -> i32 {
    7
}

fn plus_one(x: i32) -> i32 {
    x + 1
}