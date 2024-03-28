pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_string(value: String) -> Result<String, String> {
        Ok(value)
    }
}
