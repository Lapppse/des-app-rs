use bitvec::prelude::*;
use itertools::Itertools;
use std::mem::swap;
use std::str::FromStr;

use crate::traits::ToLowerHex;

use super::traits::{FromHexStr, ToUpperHex};
use super::{Error, MainKey, Result, ShiftDirection, ShiftSchemes};
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Block {
    data: BitVec,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self
            .data
            .to_string()
            .trim_matches(['[', ']'])
            .split(", ")
            .collect::<String>();
        write!(f, "{}", formatted)
    }
}

impl FromStr for Block {
    type Err = super::Error;

    /// passed str should be 64 chars long
    /// treats non-zero values as true
    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 64 {
            return Err(Error::InvalidIterableLength {
                expected: 64,
                got: s.len(),
            });
        }
        let mut data = BitVec::new();
        for ch in s.chars() {
            data.push(ch == '1');
        }
        Ok(Self { data })
    }
}

impl FromHexStr for Block {
    /// passed str should be 16 hex chars long
    fn from_hex_str(s: &str) -> Result<Self> {
        if s.len() != 16 {
            return Err(Error::InvalidIterableLength {
                expected: 16,
                got: s.len(),
            });
        }
        let data_num =
            u64::from_str_radix(s, 16).map_err(|_| Error::StringParseError(s.to_string()))?;
        let s = format!("{data_num:0>64b}");
        Self::from_str(s.as_str())
    }
}

impl ToUpperHex for Block {
    fn to_upper_hex(&self) -> String {
        let result = u64::from_str_radix(self.to_string().as_str(), 2).unwrap();
        format!("{:0>16X}", result)
    }
}

impl ToLowerHex for Block {
    fn to_lower_hex(&self) -> String {
        let result = u64::from_str_radix(self.to_string().as_str(), 2).unwrap();
        format!("{:0>16X}", result)
    }
}

impl Block {
    /// passed bitvec should contain 64 bits
    pub fn new(data: BitVec) -> Result<Self> {
        if data.len() != 64 {
            return Err(Error::InvalidIterableLength {
                expected: 64,
                got: data.len(),
            });
        }
        Ok(Self { data })
    }

    pub fn as_bitvec(&self) -> &BitVec {
        &self.data
    }

    /// returns inner BitVec consuming Self
    pub fn into_bitvec(self) -> BitVec {
        let result = self.data.to_owned();
        drop(self);
        result
    }

    pub fn encode(&self, key: &MainKey) -> Result<Self> {
        let data = ShiftSchemes::IP.shift(self.as_bitvec().to_owned())?;
        let (left, right) = data.split_at(32);
        let mut left = left.to_owned();
        let mut right = right.to_owned();
        for round in 1..=16 {
            left ^= self.f(
                right.clone(),
                key.get_round_key(round, ShiftDirection::Left)?,
            )?;
            swap(&mut left, &mut right);
        }
        right.extend(left);
        Self::new(ShiftSchemes::IP1.shift(right)?)
    }

    pub fn decode(&self, key: &MainKey) -> Result<Self> {
        let data = ShiftSchemes::IP.shift(self.as_bitvec().to_owned())?;
        let (left, right) = data.split_at(32);
        let mut left = left.to_owned();
        let mut right = right.to_owned();

        for round in 1..=16 {
            left ^= self.f(
                right.clone(),
                key.get_round_key(round, ShiftDirection::Right)?,
            )?;
            swap(&mut left, &mut right);
        }
        right.extend(left);
        Self::new(ShiftSchemes::IP1.shift(right)?)
    }

    fn f(&self, right: BitVec, key: MainKey) -> Result<BitVec> {
        let key = key.into_bitvec();
        let right = ShiftSchemes::E.shift(right)? ^ key;

        let blocks = right.chunks(6).map(|it| it.to_owned());
        let schemes = ShiftSchemes::get_s_schemes();
        let right = blocks
            .zip(schemes)
            .map(|(block, scheme)| Self::block_s_scheme_shift(block, *scheme))
            .concat();

        ShiftSchemes::P.shift(right)
    }

    fn block_s_scheme_shift(block: BitVec, s_scheme: ShiftSchemes) -> BitVec {
        let scheme = s_scheme.as_slice();
        let pos = Self::block_to_pos(block) as usize;
        let block = BitVec::from_element(scheme[pos]);
        let (block, _) = block.split_at(4);
        let mut block = block.to_owned();
        block.reverse();
        block.to_owned()
    }

    /// returns block's value's position on ShiftSchemes::S(1-8) schemes
    fn block_to_pos(block: BitVec) -> u8 {
        let i_parts = [block[0], block[5]];
        let i_parts = i_parts.map(|it| (it as u8).to_string()).concat();
        let j_parts = [block[1], block[2], block[3], block[4]];
        let j_parts = j_parts.map(|it| (it as u8).to_string()).concat();
        let i_pos = u8::from_str_radix(&i_parts, 2)
            .map_err(|_| Error::StringParseError(i_parts))
            .unwrap();
        let j_pos = u8::from_str_radix(&j_parts, 2)
            .map_err(|_| Error::StringParseError(j_parts))
            .unwrap();
        j_pos + i_pos * 16 // a row is 16 nums long hence i_pos * 16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_to_pos() -> Result<()> {
        let scheme = ShiftSchemes::S1.as_slice();
        let block = BitVec::from(bits![usize, LocalBits; 1, 0, 0, 1, 0, 1]);
        let pos = Block::block_to_pos(block) as usize;
        let block = scheme[pos];
        assert_eq!(block, 8);

        let scheme = ShiftSchemes::S7.as_slice();
        let block = BitVec::from(bits![usize, LocalBits; 0, 1, 1, 0, 1, 1]);
        let pos = Block::block_to_pos(block) as usize;
        let block = scheme[pos];
        let block: BitVec<usize, LocalBits> = BitVec::from_element(block);
        let (block, _) = block.split_at(4);
        let should_be = BitVec::from(bits![usize, LocalBits; 1, 1, 1, 1]);
        assert_eq!(block, should_be);

        Ok(())
    }

    #[test]
    fn test_block_s_shift() -> Result<()> {
        let scheme = ShiftSchemes::S3;
        let block = MainKey::from_str("110010")?.into_bitvec();
        let block = Block::block_s_scheme_shift(block, scheme);
        assert_eq!(block, MainKey::from_str("0001")?.into_bitvec());

        Ok(())
    }
}
