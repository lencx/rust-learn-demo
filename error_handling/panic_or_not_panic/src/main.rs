extern crate rand;

use std::net::IpAddr;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

#[derive(Debug)]
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    home_ip();
    guess_game();
}

fn guess_game() {
    println!("------ Guessing Game --------");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("Please input your guess. {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => Guess::new(num).value(),
            Err(_) => {
                println!("Guess value must be between 1 and 100, got {}", guess);
                continue
            },
        };

        match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal => {
               println!("You win!");
               break
           },
        }
    }
}


fn home_ip() {
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("home IP: {:?}", home);
}