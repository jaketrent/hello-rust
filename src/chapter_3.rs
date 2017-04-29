use serde_json::{self, Value, Result};

pub fn parse_input_to_json_value(input: &str) -> Result<Value> {
    serde_json::from_str(input)
}

pub fn get_meaning_of_life(input: &str) -> Result<i64> {
    match parse_input_to_json_value(input) {
        Ok(json) => Ok(json["meaningOfLife"].as_i64().unwrap()),
        Err(e) => Err(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_json() {
        assert!(parse_input_to_json_value("42").is_ok())
    }

    #[test]
    fn parse_invalid_json() {
        assert!(parse_input_to_json_value("'asdf'").is_err());
    }

    #[test]
    fn test_get_meaning_of_life() {
        match get_meaning_of_life(r#"{"meaningOfLife": 42}"#) {
            Ok(result) => assert_eq!(result, 42),
            Err(_) => assert!(false)
        }
    }
}

// let json = parse_input_to_json_value(input)?;
// Ok(json["meaningOfLife"].as_i64().unwrap())
