// scalar & compound

fn main() {
    // addition
    let sum = 5 + 8;
    println!("sum: {}", sum);

    // subtraction
    let difference = 83.1 - 14.8;
    println!("difference: {}", difference);

    // multiplication
    let product = 32 * 17;
    println!("product: {}", product);

    // division
    let quotient = 37.98 / 12.15;
    println!("quotient: {}", quotient);

    // remainder
    let remainder = 71 % 6;
    println!("remainder: {}", remainder);

    // booleans
    let t = true;
    let f: bool = false;
    println!("{}, {}", t, f);

    // characters
    let emoji = '🙈';
    let char_a = 'a';
    println!("{}, {}", emoji, char_a);

    println!("--------------------------------------");

    /* compound types */
    // tuple
    let tup: (i32, f64, i16) = (15653, 4.23546, 156);
    let (_x, _y, _z) = tup;
    println!("The value of y is: {}", _y);
    println!("The value of x is: {}", tup.0);

    // array
    let arr = [1, 14, -2, 17];
    println!("{}", arr[3]);
}
