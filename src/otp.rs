use base32::{decode, Alphabet};
use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::base32string::Base32String;

type HmacSha1 = Hmac<Sha1>;

pub fn generate(base32_secret: &Base32String, message: &[u8; 8]) -> u32 {
    let secret = decode_secret(base32_secret.to_str());
    let mac = generate_mac_value(&secret, message);
    generate_code(&mac)
}

fn decode_secret(base32_secret: &str) -> Vec<u8> {
    match decode(Alphabet::Rfc4648 { padding: true }, base32_secret) {
        Some(s) => s,
        _ => panic!("Failed to decode"),
    }
}

fn generate_mac_value(secret: &[u8], message: &[u8; 8]) -> [u8; 20] {
    let mut mac = HmacSha1::new_from_slice(&secret).unwrap();
    mac.update(message);
    mac.finalize().into_bytes().into()
}

fn generate_code(hmac: &[u8; 20]) -> u32 {
    let offset = (hmac[19] & 0xf) as usize;
    ((hmac[offset] & 0x7f) as u32) << 24
        | ((hmac[offset + 1] & 0xff) as u32) << 16
        | ((hmac[offset + 2] & 0xff) as u32) << 8
        | ((hmac[offset + 3] & 0xff) as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_given_secret_and_message_should_return_otp_code() {
        let secret = match Base32String::from("TEST====") {
            Ok(secret) => secret,
            Err(_) => panic!("Failed to create test case"),
        };
        let message = 123456_u64.to_be_bytes();

        let code = generate(&secret, &message);

        assert_eq!(code, 768744478);
    }

    #[test]
    fn generate_mac_value_given_secret_and_message_should_return_mac_value() {
        let secret = [0x99, 0x25];
        let message = 123456_u64.to_be_bytes();

        let mac = generate_mac_value(&secret, &message);

        assert_eq!(
            mac,
            [
                0xd7, 0x69, 0x81, 0x1a, 0xf9, 0xb3, 0x1f, 0xb8, 0x6b, 0xc9, 0x86, 0x8a, 0x38, 0x0d,
                0xad, 0xd2, 0x1c, 0x1e, 0xb0, 0xae
            ]
        )
    }
}
