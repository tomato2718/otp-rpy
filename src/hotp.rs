use crate::base32string::Base32String;
use crate::otp;

pub fn generate(base32_secret: &Base32String, count: &u64) -> u32 {
    otp::generate(base32_secret, &count.to_be_bytes())
}

pub fn validate(
    code: &str,
    base32_secret: &Base32String,
    count: &u64,
    tolerance: Option<u64>,
) -> bool {
    let t = tolerance.unwrap_or(0);
    for c in count - t..count + t + 1 {
        let valid_code = generate(base32_secret, &c).to_string();
        if code == &valid_code[valid_code.len() - code.len()..] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_given_secret_and_count_return_code() {
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let count = 123456;

        let code = generate(&secret, &count);

        assert_eq!(code, 768744478);
    }

    #[test]
    fn validate_given_valid_code_secret_and_count_return_true() {
        let code = "744478";
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let count = 123456;

        assert!(validate(&code, &secret, &count, None));
    }

    #[test]
    fn validate_given_valid_code_secret_and_count_in_tolerance_return_true() {
        let code = "278016";
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let count = 123456;

        assert!(validate(&code, &secret, &count, Some(1)));
    }

    #[test]
    fn validate_given_invalid_code_secret_and_count_return_false() {
        let code = "744478";
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let count = 654321;

        assert!(!validate(&code, &secret, &count, None));
    }
}
