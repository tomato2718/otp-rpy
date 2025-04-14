pub struct Base32String(String);

impl Base32String {
    pub fn from(s: &str) -> Result<Self, ValidationError> {
        if s.chars()
            .all(|char| (char >= 'A' && char <= 'Z') || (char >= '2' && char <= '7'))
        {
            Ok(Base32String(s.to_string()))
        } else {
            Err(ValidationError(
                "Input contains none base32 character.".to_string(),
            ))
        }
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct ValidationError(String);

impl ValidationError {
    pub fn msg(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test_base32_string {
    use super::*;

    #[test]
    fn from_given_base32_str_should_return_instance() {
        let b32str = Base32String::from("TEST");

        assert!(b32str.is_ok());
    }

    #[test]
    fn from_given_none_base32_str_should_return_validation_error() {
        let err = Base32String::from("invalid");

        assert!(err.is_err());
    }
}
