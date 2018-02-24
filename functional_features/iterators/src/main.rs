extern crate iterators;

use iterators::Counter;

fn main() {
    demo_1();
    demo_2();
    demo_iter();
}

fn demo_1() {
    let v1 = vec![1, 5, 7, 10, 3];
    let mut v1_iter = v1.iter();

    println!("v1 next: {:?}", v1_iter.next());
    println!("v1 next: {:?}", v1_iter.next());
    println!("v1 next: {:?}", v1_iter.next());
    
    for v in v1_iter {
        println!("Got: {}", v);
    }

    let v2 = vec![2, 5, 7, 2, 8];
    let sum: i32 = v2.iter().sum();
    println!("sum: {}", sum)
}

fn demo_2() {
    let v: Vec<i32> = vec![2, 4, 5];
    let add_one: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", add_one);
}

// #[derive(Debug)]
// struct Counter {
//     count: u32,
// }
// impl Counter {
//     fn new() -> Counter {
//         Counter {count: 0}
//     }
// }
// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;

//         if self.count < 6 {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

fn demo_iter() {
    println!("----------------------");
    let sum: u32 = Counter::new()
                    .zip(Counter::new().skip(1))
                    .map(|(a, b)| a * b)                       .filter(|x| x % 3 == 0)
                    .sum();
    println!("sum: {}", sum);
    // println!("zip: {:#?}", Counter::new().zip(Counter::new().skip(1)));
    // println!("map: {:#?}", Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b));
}