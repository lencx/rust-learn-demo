fn main() {
    // println!("Hello, world!");
    max_num();

    let num = vec![1, 4, 18, 5, 66];
    println!("result: {}", largest_num(&num));

    let letter = vec!['c', 'g', 'z', 'a', 'h'];
    println!("result: {}", largest_char(&letter));

    // let str_list = vec!["c", "a", "l", "Z", "o"];
    // println!("result: {}", largest(&str_list));
    coordinate();
}

fn max_num() {
    let num_list = vec![32, 12, 45, 23, 19];
    let mut largest = num_list[0];
    for number in num_list {
        if number > largest {
            largest = number
        }
    }
    println!("The largest number is {}", largest);
}

fn largest_num(list: &[i32]) -> i32 {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item
        }
    }
    max
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
//     fn y(&self) -> &T {
//         &self.y
//     }
// }
impl<T, U> Point<T, U> {
    fn mixup_y<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         println!("x: {:?}", self.x);
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

fn coordinate() {
    println!("------------ Coordinate -------------");
    let int_ponit = Point {x: 12, y: 10};
    let float_ponit = Point {x: 8.4, y: -3.5};

    println!("Coordinate: {:#?}, {:#?}", int_ponit, float_ponit);

    let p1 = Point {x: -10, y: 15};
    let p2 = Point {x: 'c', y: -20};
    // mixupY: Point { x: -10, y: -20 }
    println!("mixupY: {:?}", p1.mixup_y(p2));
    // println!("distance => {:?}", p1.distance_from_origin()); // 9.099999
}