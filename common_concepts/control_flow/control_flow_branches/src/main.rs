fn main() {
    demo_one();
    demo_two();
    demo_three();
    demo_four();
}

fn demo_one() {
    let number = 5;
    if number > 8 {
        println!("deme 1: condition was true!");
    } else {
        println!("deme 1: condition was false!");
    }
}

fn demo_two() {
    let number = 4;
    if number != 0 {
        println!("number was something other than zero");
    }
}

fn demo_three() {
    let number = 76;
    if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 5 == 0 {
        println!("number is divisible by 5");
    } else {
        println!("number is not divisible by 3, 4 or 5");
    }
}

fn demo_four() {
    let condition = true;
    let number = if condition {
        'c'
    } else {
        'a'
    };

    println!("The value of number is: {}", number);
}