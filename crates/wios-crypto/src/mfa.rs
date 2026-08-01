//! MFA/Passkey abstractions — TOTP and recovery codes.

use ring::hmac;
use std::time::{SystemTime, UNIX_EPOCH};
use wios_core::error::WiosResult;

/// TOTP (Time-based One-Time Password) generator/verifier (RFC 6238).
pub struct Totp;

impl Totp {
    /// Generate a random 20-byte TOTP secret.
    pub fn generate_secret() -> Vec<u8> {
        let mut secret = vec![0u8; 20];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut secret);
        secret
    }

    /// Generate a TOTP code for the current time step.
    pub fn generate(secret: &[u8], time_step: u64, digits: u32) -> WiosResult<String> {
        let counter = current_counter(time_step);
        Self::hotp(secret, counter, digits)
    }

    /// Verify a TOTP code, allowing ±1 time step skew.
    pub fn verify(secret: &[u8], code: &str, time_step: u64, digits: u32) -> WiosResult<bool> {
        let counter = current_counter(time_step);
        for offset in [-1i64, 0, 1] {
            let c = (counter as i64 + offset) as u64;
            let expected = Self::hotp(secret, c, digits)?;
            if constant_time_eq(expected.as_bytes(), code.as_bytes()) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// HOTP (RFC 4226) — HMAC-based One-Time Password.
    fn hotp(secret: &[u8], counter: u64, digits: u32) -> WiosResult<String> {
        let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, secret);
        let msg = counter.to_be_bytes();
        let tag = hmac::sign(&key, &msg);
        let hash = tag.as_ref();

        let offset = (hash[hash.len() - 1] & 0x0F) as usize;
        let binary = ((hash[offset] as u32 & 0x7F) << 24)
            | ((hash[offset + 1] as u32) << 16)
            | ((hash[offset + 2] as u32) << 8)
            | (hash[offset + 3] as u32);

        let modulus = 10u32.pow(digits);
        Ok(format!("{:0>width$}", binary % modulus, width = digits as usize))
    }

    /// Encode secret as Base32 for QR code / authenticator app.
    pub fn secret_to_base32(secret: &[u8]) -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        let mut result = String::new();
        let mut buffer: u64 = 0;
        let mut bits = 0;
        for &byte in secret {
            buffer = (buffer << 8) | byte as u64;
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                result.push(ALPHABET[((buffer >> bits) & 0x1F) as usize] as char);
            }
        }
        if bits > 0 {
            buffer <<= 5 - bits;
            result.push(ALPHABET[(buffer & 0x1F) as usize] as char);
        }
        result
    }
}

/// Recovery code manager.
pub struct RecoveryCodes;

impl RecoveryCodes {
    /// Generate a set of recovery codes.
    pub fn generate(count: usize) -> Vec<String> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        (0..count)
            .map(|_| {
                let mut bytes = [0u8; 5];
                rng.fill_bytes(&mut bytes);
                let code: u64 = bytes.iter().fold(0u64, |acc, &b| (acc << 8) | b as u64);
                format!("{:010}", code % 10_000_000_000)
            })
            .collect()
    }

    /// Format codes as "XXXXX-XXXXX" for display.
    pub fn format_code(code: &str) -> String {
        if code.len() == 10 {
            format!("{}-{}", &code[..5], &code[5..])
        } else {
            code.to_string()
        }
    }
}

fn current_counter(time_step: u64) -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / time_step
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (&x, &y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totp_generate_verify() {
        let secret = Totp::generate_secret();
        let code = Totp::generate(&secret, 30, 6).unwrap();
        assert_eq!(code.len(), 6);
        assert!(Totp::verify(&secret, &code, 30, 6).unwrap());
    }

    #[test]
    fn test_totp_wrong_code() {
        let secret = Totp::generate_secret();
        assert!(!Totp::verify(&secret, "000000", 30, 6).unwrap());
    }

    #[test]
    fn test_base32_encoding() {
        let encoded = Totp::secret_to_base32(b"Hello!");
        assert!(!encoded.is_empty());
        assert!(encoded.chars().all(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(c)));
    }

    #[test]
    fn test_recovery_codes() {
        let codes = RecoveryCodes::generate(8);
        assert_eq!(codes.len(), 8);
        for code in &codes {
            assert_eq!(code.len(), 10);
            let formatted = RecoveryCodes::format_code(code);
            assert!(formatted.contains('-'));
        }
    }
}
