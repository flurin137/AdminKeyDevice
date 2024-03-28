use regex::Regex;

pub struct Validator {
    regex: Regex,
}

impl Validator {
    pub fn new() -> Self {
        let regex = Regex::new(r"^[\w\d]+$").unwrap();
        Self { regex }
    }

    pub fn validate(&self, value: String) -> Result<String, String> {
        if value.len() > 64 {
            return Err("String is too long".to_owned());
        }

        if !self.regex.is_match(&value) {
            return Err("The string contains invalid characters".to_owned());
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_long_string() {
        let validator = Validator::new();

        let input =
            "dhfdskjhdsggfdskfhgasdkjghfjdsghfadshgfjkasghdfadsgasghdfjaghkdsfahfk".to_string();

        let result = validator.validate(input);

        assert_eq!(Err("String is too long".to_owned()), result)
    }

    #[test]
    fn test_invalid_chars() {
        let validator = Validator::new();

        let input = "*/".to_string();

        let result = validator.validate(input);

        assert_eq!(
            Err("The string contains invalid characters".to_owned()),
            result
        )
    }
}
