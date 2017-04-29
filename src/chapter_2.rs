// Option built in
pub fn divide_safely(a: i32, b: i32) -> Option<i32> {
    if b == 0 { // 1
        None
    } else {
        Some(a / b)
    }
}

pub fn divide_with_no_remorse(a: i32, b: i32) -> i32 {
    // unwrap will panic if it doesn't work; otherwise, unwraps value
    divide_safely(a, b).unwrap() // 2
}

// build with cargo build in terminal
// run with cargo test
#[cfg(test)]
mod tests {
    // make everything that's public in the super module (this file), available here
    use super::*;

    #[test]
    fn test_divide_safely() {
        assert_eq!(divide_safely(4, 2), Some(2));
        assert_eq!(divide_safely(8, 3), Some(2));
        assert_eq!(divide_safely(4, 0), None);
    }

    #[test]
    fn test_divide_with_no_remorse_successfully() {
        assert_eq!(divide_with_no_remorse(4, 2), 2);
        assert_eq!(divide_with_no_remorse(8, 3), 2);
    }

    #[test]
    #[should_panic]
    fn test_divide_with_no_remorse_unsuccessfully() {
        divide_with_no_remorse(4, 0);
    }
}
