pub fn meaning_of_life() -> i32 {
    let x = 42;
    x // leave ; off for making this the return expression
}

// tests included inline with the src
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_meaning() {
        assert_eq!(42, meaning_of_life());
    }
}
