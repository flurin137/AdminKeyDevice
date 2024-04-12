pub trait LanguageMapper {
    fn sanitize(&self, value: String) -> String;
}

pub struct AmericanEnglishLanguageMapper;
pub struct SwissGermanLanguageMapper;

impl SwissGermanLanguageMapper {
    pub fn new_boxed() -> Box<SwissGermanLanguageMapper> {
        Box::new(SwissGermanLanguageMapper)
    }
}

impl LanguageMapper for SwissGermanLanguageMapper {
    fn sanitize(&self, value: String) -> String {
        value
            .replace('z', "_")
            .replace('y', "z")
            .replace('_', "y")
            .replace('Z', "_")
            .replace('Y', "Z")
            .replace('_', "Y")
    }
}

impl LanguageMapper for AmericanEnglishLanguageMapper {
    fn sanitize(&self, value: String) -> String {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_langiage_specific_chars() {
        let validator = SwissGermanLanguageMapper;

        let input = "Yaahuz".to_string();

        let result = validator.sanitize(input);

        assert_eq!("Zaahuy".to_string(), result)
    }
}
