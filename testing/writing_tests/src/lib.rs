#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
        assert_ne!(3 * 5, 15); // FAILED
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn sum() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {length: 8, width: 20};
        let smaller = Rectangle {length: 5, width: 19};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {length: 10, width: 17};
        let smaller = Rectangle {length: 5, width: 16};

        assert!(smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let r = greeting("Carol");
        assert!(
            r.contains("Carol"),
            "Greeting did not contain name, value was `{}`", r
        );
    }

    #[test]
    #[should_panic(expected="Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(199);
    }
}


#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
    // a + 3
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("Hello")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        // if value < 1 || value > 100 {
            // panic!("Guess value must be between 1 and 100, got {}.", value);
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }

        Guess {
            value
        }
    }
}