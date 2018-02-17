pub mod test_modules {
    pub mod series {
        pub mod of {
            pub fn nested_mod() {
                println!("nested modules");
            }
        }
    }
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}


// use test_modules::series::of; // ①
use test_modules::series::of::nested_mod; // ②

// use TrafficLight::{Red, Green}; // ③
use TrafficLight::*; // ④

fn main() {
    // test_modules::series::of::nested_mod();
    // of::nested_mod(); // ①
    nested_mod(); // ②

    let red = Red;
    let green = Green;
    let yellow = TrafficLight::Yellow;
    println!("TrafficeLight: {:?}, {:?}, {:?}", red, green, yellow);
}