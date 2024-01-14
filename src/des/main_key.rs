use super::{Error, Result, ShiftDirection, ShiftSchemes};
use bitvec::prelude::*;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MainKey {
    // FIXME: pub or not?
    // TODO: MainKey as_slice and as_bitvec and len
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

    /// Accepts both binary and hex strings
    fn from_str(s: &str) -> Result<Self> {
        let mut key = BitVec::new();
        if s.to_lowercase().contains(['a', 'b', 'c', 'd', 'e', 'f']) {
            let key_num =
                u64::from_str_radix(s, 16).map_err(|_| Error::StringParseError(s.to_string()))?;
            let s = format!("{key_num:0>width$b}", width = s.len() * 4);

            // FIXME: repetitive code
            for ch in s.as_str().chars() {
                key.push(ch != '0');
            }
            return Ok(Self { key });
        }

        for ch in s.chars() {
            key.push(ch != '0');
        }
        Ok(Self { key })
    }
}

impl MainKey {
    pub fn new(key: BitVec) -> Self {
        Self { key }
    }

    pub fn to_bitvec(&self) -> BitVec {
        self.key.clone()
    }

    pub fn as_bitvec(&self) -> BitVec {
        let result = self.key.to_owned();
        let _ = self;
        result
    }

    /// Returns uppercase hex string
    pub fn to_hex_string(&self) -> String {
        let result = u64::from_str_radix(self.to_string().as_str(), 2).unwrap(); // FIXME:
        format!("{:0>width$X}", result, width = self.key.len() / 4)
    }

    /// Returns new key with bits shifted according to given scheme. Trims the key if the scheme is shorter
    pub fn shift_scheme(&self, scheme: ShiftSchemes) -> Result<Self> {
        let needed_len = scheme.as_slice().len();
        if self.key.len() < needed_len {
            return Err(Error::InvalidIterableLength {
                expected: needed_len,
                got: self.key.len(),
            });
        }
        let key = scheme.shift(self.as_bitvec())?;
        Ok(Self::new(key))
    }

    /// Returns new round shifted key (doesn't mutate self). Round should be 1..=16
    pub fn shift_round(&self, round: u8, direction: ShiftDirection) -> Result<Self> {
        // FIXME: repetitive code if round is in bounds
        if !(1..=16).contains(&round) {
            return Err(Error::InvalidRound(round));
        }

        let round_shift = direction.get_round_shift(round)? as usize;
        let key = self.key.clone();
        let (left, right) = key.split_at(key.len() / 2);
        let mut left = left.to_owned().to_bitvec();
        let mut right = right.to_owned();

        left.rotate_left(round_shift);
        right.rotate_left(round_shift);
        left.extend(right);

        Ok(Self::new(left))
    }

    /// Combines shifting by PC1, round shifting and shifting by PC2
    /// Should be preferred against using 3 functions separately
    pub fn get_round_key(&self, round: u8, direction: ShiftDirection) -> Result<Self> {
        self.shift_scheme(ShiftSchemes::PC1) // FIXME: use standartized Shiftschemes::shift
            .and_then(|key| key.shift_round(round, direction))
            .and_then(|key| key.shift_scheme(ShiftSchemes::PC2))
    }
}
