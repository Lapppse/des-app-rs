use super::traits::{FromHexStr, ToLowerHex, ToUpperHex};
use super::{Error, Result, ShiftDirection, ShiftSchemes};
use bitvec::prelude::*;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MainKey {
    key: BitVec,
}

impl fmt::Display for MainKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self
            .key
            .to_string()
            .trim_matches(['[', ']'])
            .split(", ")
            .collect::<String>();
        write!(f, "{}", formatted)
    }
}

impl FromStr for MainKey {
    type Err = super::Error;

    /// treats non-zero values as true
    fn from_str(s: &str) -> Result<Self> {
        let mut key = BitVec::new();
        for ch in s.chars() {
            key.push(ch == '1');
        }
        Ok(Self::new(key))
    }
}

impl FromHexStr for MainKey {
    fn from_hex_str(s: &str) -> Result<Self> {
        let key_num =
            u64::from_str_radix(s, 16).map_err(|_| Error::StringParseError(s.to_string()))?;
        let s = format!("{key_num:0>width$b}", width = s.len() * 4);
        Self::from_str(s.as_str())
    }
}

impl ToUpperHex for MainKey {
    /// returns zero-padded uppercase hex string
    /// if you want to have unpadded hex string, consider converting into bitvec and calling this method on the result
    fn to_upper_hex(&self) -> String {
        let converted = self.key.to_upper_hex();
        if converted.len() % 16 == 0 {
            return converted;
        }
        format!(
            "{:0>width$}",
            converted,
            width = converted.len() + (16 - converted.len() % 16)
        )
    }
}

impl ToLowerHex for MainKey {
    /// returns zero-padded uppercase hex string
    /// if you want to have unpadded hex string, consider converting into bitvec and calling this method on the result
    fn to_lower_hex(&self) -> String {
        let converted = self.key.to_lower_hex();
        if converted.len() % 16 == 0 {
            return converted;
        }
        format!(
            "{:0>width$}",
            converted,
            width = converted.len() + (16 - converted.len() % 16)
        )
    }
}

impl MainKey {
    pub fn new(key: BitVec) -> Self {
        Self { key }
    }

    pub fn as_bitvec(&self) -> &BitVec {
        &self.key
    }

    /// returns inner BitVec consuming Self
    pub fn into_bitvec(self) -> BitVec {
        let result = self.key.to_owned();
        drop(self);
        result
    }

    fn shift_scheme(&mut self, scheme: ShiftSchemes) -> Result<()> {
        let needed_len = scheme.as_slice().len();
        if self.key.len() < needed_len {
            return Err(Error::InvalidIterableLength {
                expected: needed_len,
                got: self.key.len(),
            });
        }

        self.key = scheme.shift(self.key.clone())?;
        Ok(())
    }

    /// returns new round shifted key (doesn't mutate self). u8 should be 1..=16
    fn shift_round(&mut self, round: u8, direction: ShiftDirection) -> Result<()> {
        let round_shift = direction.get_round_shift(round)? as usize;
        let (left, right) = self.key.split_at(self.key.len() / 2);
        let mut left = left.to_owned();
        let mut right = right.to_owned();

        left.rotate_left(round_shift);
        right.rotate_left(round_shift);
        left.extend(right);

        self.key = left;
        Ok(())
    }

    /// returns new instance of MainKey
    pub fn get_round_key(&self, round: u8, direction: ShiftDirection) -> Result<Self> {
        let mut new_key = self.clone();
        new_key.shift_scheme(ShiftSchemes::PC1)?;
        new_key.shift_round(round, direction)?;
        new_key.shift_scheme(ShiftSchemes::PC2)?;
        Ok(new_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pc1_shift() -> Result<()> {
        let mut key = MainKey::from_hex_str("AABB09182736CCDD")?;
        key.shift_scheme(ShiftSchemes::PC1)?;
        assert_eq!(key, MainKey::from_hex_str("C3C033A33F0CFA")?);
        Ok(())
    }
}
