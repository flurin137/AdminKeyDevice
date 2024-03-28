pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, value: String) -> Result<String, String> {
        if value.len() > 64 {
            return Err("String is too long".to_owned());
        }

        Ok(value)
    }
}
