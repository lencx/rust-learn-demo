fn main() {
    loop_fn();
    while_fn();
    through_a_collection();
    for_fn();
    for_rev_fn();
}

fn loop_fn() {
    loop {
        println!("again!");
        break;
    }
}

fn while_fn() {
    let mut number = 5;

    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn through_a_collection() {
    let a = [9, 5, 3, 7, 4];
    let mut index = 0;

    while index < 4 {
        println!("The value is: {}", a[index]);
        index = index + 1;
    }
}

fn for_fn() {
    let a = ['a', 'c', 'o', 'd', 'e'];

    for item in a.iter() {
        println!("the value is: {}", item);
    }
}

fn for_rev_fn() {
    for number in (0..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}