extern crate trait_objects;
// use trait_objects::Draw;
// use trait_objects::{Screen, Button};
// use trait_objects::clone;

fn main() {
    let a = String::from("hhhh");
    println!("a: {}", a);
    let b = a.clone();
    println!("b: {}", b);
    let a2 = vec![2, 4, 6];
    println!("a2: {:?}", a2);
    let b2 = a2.clone();
    println!("b2: {:?}", b2);

    // let screen = Screen {
    //     components: vec![
    //         Box::new(String::from("HI")),
    //     ],
    // };

    // screen.run();
    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No")
    //             ],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("Ok")
    //         }),
    //     ],
    // };

    // screen.run();
}


// #[derive(Debug)]
// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {

//     }
// }
