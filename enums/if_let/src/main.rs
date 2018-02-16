#[derive(Debug)]
enum Coin {
    // Penny,
    Nickel,
    // Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    // Alabama,
    Alaska,
    // ... etc
}

fn main() {
    let some_u8_val = Some(0u8);
    match some_u8_val {
        Some(3) => println!("three"),
        _ => (),
    }

    let val2 = Some(5);
    if let Some(4) = val2 {
        println!("four");
    } else {
        println!("other");
    }

    coin_demo();
    coin_if_let();
}

fn coin_demo() {
    // let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    // let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        // _ => count += 1,
        _ => (),
    }
    // println!("count: {}", count);
}

fn coin_if_let() {
    let mut count = 0;
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Nickel;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state)
    } else {
        count += 1;
        println!("other!!! count: {}", count)
    }
}