use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    fn_1();
    // fn_2();
    fn_3();
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[derive(Debug)]
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn fn_1() {
    let x = 6;
    let equal_to_x = |z| z == x;
    let y = 8;
    assert!(equal_to_x(y)); // false
    // println!("{}", equal_to_x(y));
}
fn fn_3() {
    let x = vec![1, 3, 5];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);

    let y = vec![1, 3, 5];
    assert!(equal_to_x(y));
    // println!("{}", equal_to_x(y));
}
// fn fn_2() {
//     let x = 6;
//     fn equal_to_x(z: i32) -> bool {z == x};
//     let y = 8;
//     assert!(equal_to_x(y)); // false
// }

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(2);
    let v2 = c.value(5);
    assert_eq!(v2, 5); // left: 2, right: 5
}