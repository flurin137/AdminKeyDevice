use regex::Regex;

use crate::sanitizer::LanguageMapper;

pub struct Validator {
    regex: Regex,
    language_mapper: Box<dyn LanguageMapper>,
}

impl Validator {
    pub fn new(language_mapper: Box<dyn LanguageMapper>) -> Self {
        let regex = Regex::new(r"^[\w\d]+$").unwrap();
        Self {
            regex,
            language_mapper,
        }
    }

    pub fn validate(&self, value: String) -> Result<String, String> {
        if value.len() > 64 {
            return Err("String is too long".to_owned());
        }

        if value.len() == 0 {
            return Err("The string is empty".to_owned());
        }

        if !self.regex.is_match(&value) {
            return Err("The string contains invalid characters".to_owned());
        }

        Ok(value)
    }

    pub fn sanitize(&self, value: String) -> String {
        self.language_mapper.sanitize(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::sanitizer::SwissGermanLanguageMapper;

    use super::*;

    #[test]
    fn test_too_long_string() {
        let validator = Validator::new(SwissGermanLanguageMapper::new_boxed());

        let input =
            "dhfdskjhdsggfdskfhgasdkjghfjdsghfadshgfjkasghdfadsgasghdfjaghkdsfahfk".to_string();

        let result = validator.validate(input);

        assert_eq!(Err("String is too long".to_owned()), result)
    }

    #[test]
    fn test_invalid_chars() {
        let validator = Validator::new(SwissGermanLanguageMapper::new_boxed());

        let input = "*/".to_string();

        let result = validator.validate(input);

        assert_eq!(
            Err("The string contains invalid characters".to_owned()),
            result
        )
    }
}
