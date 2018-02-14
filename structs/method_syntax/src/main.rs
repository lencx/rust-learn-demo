#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        println!("self: {:?}", self);
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn sq(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let r1 = Rectangle {width: 15, height: 30};
    let r2 = Rectangle {width: 12, height: 20};
    let r3 = Rectangle {width: 30, height: 20};

    println!("r1: {:#?}", r1);
    println!("The area of r1 is {} square pixels.", r1.area());
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r2 hold r3? {}", r2.can_hold(&r3));
    let sq = Rectangle::sq(10);
    println!("sq: {:?}", sq);
}
