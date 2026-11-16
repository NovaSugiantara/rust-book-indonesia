use crate::error::AppError;
use rand::prelude::*;
use serde::Serialize;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

const ALPHABET: &[u8] = b"abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const MIN_LENGTH: usize = 4;
const MAX_LENGTH: usize = 12;
const GENERATED_LENGTH: usize = 7;

/// Kode pendek yang aman dipakai sebagai path URL.
///
/// ```
/// use shortener::ShortCode;
///
/// let code = ShortCode::parse("promo").unwrap();
/// assert_eq!(code.as_ref(), "promo");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct ShortCode(String);

impl ShortCode {
    /// Memvalidasi kode dari input pengguna.
    pub fn parse(input: &str) -> Result<Self, AppError> {
        let value = input.trim();

        if !(MIN_LENGTH..=MAX_LENGTH).contains(&value.len()) {
            return Err(crate::bad_request!(
                "kode harus {MIN_LENGTH} sampai {MAX_LENGTH} karakter"
            ));
        }

        if !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
        {
            return Err(crate::bad_request!(
                "kode hanya boleh berisi huruf dan angka"
            ));
        }

        Ok(Self(value.to_string()))
    }

    /// Membuat kode acak yang mudah dibaca.
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        let code = (0..GENERATED_LENGTH)
            .map(|_| {
                let index = rng.random_range(0..ALPHABET.len());
                ALPHABET[index] as char
            })
            .collect();

        Self(code)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for ShortCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for ShortCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ShortCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for ShortCode {
    type Err = AppError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_short_code() {
        assert!(ShortCode::parse("abc").is_err());
    }

    #[test]
    fn rejects_symbols() {
        assert!(ShortCode::parse("promo!").is_err());
    }

    #[test]
    fn generates_readable_code() {
        let code = ShortCode::generate();
        assert_eq!(code.as_str().len(), GENERATED_LENGTH);
        assert!(
            code.chars()
                .all(|character| character.is_ascii_alphanumeric())
        );
    }
}
