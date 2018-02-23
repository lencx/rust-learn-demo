#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn this_test_will_pass() {
        let val = prints_and_return_10(4);
        assert_eq!(10, val);
    }

    #[test]
    fn this_test_will_fail() {
        let val = prints_and_return_10(8);
        assert_eq!(7, val);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

pub fn prints_and_return_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}