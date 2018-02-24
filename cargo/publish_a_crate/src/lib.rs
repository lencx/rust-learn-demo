#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Add one to the number given.
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(9, publish_a_crate::add_two(7));
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}
