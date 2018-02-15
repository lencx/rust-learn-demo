#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

fn main() {
    // println!("Hello, world!");
    values_in_cents(Coin::Penny);
    values_in_cents(Coin::Quarter(UsState::Alabama));

    let six = Some(6);
    let num = plus_one(six);
    let none = plus_one(None);
    println!("num: {:?}; none: {:?}", num, none);

    println!("********************");
    some_u8();
}

fn values_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!!!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 3),
    }
}

fn some_u8() {
    let some_u8_val = 3;
    match some_u8_val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        _ => (), // placeholder
    }
}