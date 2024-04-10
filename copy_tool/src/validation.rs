use regex::Regex;

pub enum Language {
    DE_CH,
    EN_US,
}

pub struct Validator {
    regex: Regex,
    language: Language,
}

impl Validator {
    pub fn new(language: Language) -> Self {
        let regex = Regex::new(r"^[\w\d]+$").unwrap();
        Self { regex, language }
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

    pub fn sanitize(&self, value: String) -> String {
        return match self.language {
            Language::DE_CH => value
                .replace("z", "_")
                .replace("y", "z")
                .replace("_", "y")
                .replace("Z", "_")
                .replace("Y", "Z")
                .replace("_", "Y"),
            Language::EN_US => value,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_long_string() {
        let validator = Validator::new(Language::DE_CH);

        let input =
            "dhfdskjhdsggfdskfhgasdkjghfjdsghfadshgfjkasghdfadsgasghdfjaghkdsfahfk".to_string();

        let result = validator.validate(input);

        assert_eq!(Err("String is too long".to_owned()), result)
    }

    #[test]
    fn test_invalid_chars() {
        let validator = Validator::new(Language::DE_CH);

        let input = "*/".to_string();

        let result = validator.validate(input);

        assert_eq!(
            Err("The string contains invalid characters".to_owned()),
            result
        )
    }

    #[test]
    fn test_with_langiage_specific_chars() {
        let validator = Validator::new(Language::DE_CH);

        let input = "Yaahuz".to_string();

        let result = validator.sanitize(input);

        assert_eq!("Zaahuy".to_string(), result)
    }
}
