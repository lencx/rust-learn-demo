extern crate art;

use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let blue = PrimaryColor::Blue;

    let c = mix(red, blue);
    println!("color: {:?}", c)·;
}