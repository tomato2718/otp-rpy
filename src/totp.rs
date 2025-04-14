use crate::base32string::Base32String;
use crate::otp;

const INTERVAL: u64 = 30;

pub fn generate(base32_secret: &Base32String, now: &u64) -> u32 {
    let steps = now / INTERVAL;
    otp::generate(base32_secret, &steps.to_be_bytes())
}

pub fn validate(
    code: &str,
    base32_secret: &Base32String,
    now: &u64,
    tolerance: Option<u64>,
) -> bool {
    let t = tolerance.unwrap_or(0) * INTERVAL;
    for c in now - t..now + t + INTERVAL {
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
    fn generate_given_secret_and_current_time_return_code() {
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let now = 1726717220;

        let code = generate(&secret, &now);

        assert_eq!(code, 1280794695);
    }

    #[test]
    fn validate_given_valid_code_secret_and_current_time_return_true() {
        let code = "794695";
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let now = 1726717220;

        assert!(validate(&code, &secret, &now, None));
    }

    #[test]
    fn validate_given_valid_code_secret_and_current_time_in_tolerance_return_true() {
        let code = "027100";
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let now = 1726717220;

        assert!(validate(&code, &secret, &now, Some(1)));
    }

    #[test]
    fn validate_given_invalid_code_secret_and_current_time_return_false() {
        let code = "123456";
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let now = 1726717220;

        assert!(!validate(&code, &secret, &now, None));
    }
}
