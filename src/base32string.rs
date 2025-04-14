pub struct Base32String(String);

impl Base32String {
    const VALIDATION_ERROR: ValidationError = ValidationError("Invalid Base32 string");

    pub fn from(s: &str) -> Result<Self, ValidationError> {
        Base32String::validate_length(s)?;
        Base32String::validate_characters(s)?;
        Base32String::validate_padding(s)?;

        Ok(Base32String(s.to_string()))
    }

    fn validate_length(s: &str) -> Result<(), ValidationError> {
        if s.len() % 8 != 0 {
            return Err(Base32String::VALIDATION_ERROR);
        }
        Ok(())
    }

    fn validate_characters(s: &str) -> Result<(), ValidationError> {
        if !s
            .chars()
            .all(|c| (c >= 'A' && c <= 'Z') || (c >= '2' && c <= '7') || c == '=')
        {
            return Err(Base32String::VALIDATION_ERROR);
        }
        Ok(())
    }

    fn validate_padding(s: &str) -> Result<(), ValidationError> {
        if let Some(padding_start) = s.find('=') {
            if !(vec![1, 3, 4, 6].contains(&(s.len() - padding_start))
                && s[padding_start..].chars().all(|c| c == '='))
            {
                return Err(Base32String::VALIDATION_ERROR);
            }
        }
        Ok(())
    }

    pub fn to_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct ValidationError(&'static str);

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
        let test_cases = vec!["TESTTEST", "TESTTES=", "TESTT===", "TEST====", "TE======"];

        for s in test_cases {
            let b32str = Base32String::from(s);

            assert!(b32str.is_ok());
        }
    }

    #[test]
    fn from_given_none_base32_str_should_return_validation_error() {
        let test_cases = vec!["TEST", "TESTTE=T", "TESTTE=="];

        for s in test_cases {
            let err = Base32String::from(s);

            assert!(err.is_err());
        }
    }
}
