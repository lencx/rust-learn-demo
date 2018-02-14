#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = rect_area(30, 40);
    println!("The area of the rectangle is {} square pixels.", rect1);

    let rect2 = rect_area2((20, 14));
    println!("rect2: {}", rect2);

    let _r = Rect{width: 10, height: 15};
    println!("_r: {:?}", _r);
    println!("_r: {:#?}", _r);
    let rect3 = area(&_r);
    println!("rect3: {}", rect3);
}

fn rect_area(width: u32, height: u32) -> u32 {
    width * height
}
fn rect_area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}