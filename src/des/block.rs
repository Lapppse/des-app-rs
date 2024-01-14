use bitvec::prelude::*;
use itertools::Itertools;
use std::fmt;
use std::mem::swap;
use std::str::FromStr;

use super::{Error, MainKey, Result, ShiftDirection, ShiftSchemes};

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

    /// Converts from binary str
    fn from_str(s: &str) -> Result<Self> {
        let mut data = BitVec::new();
        for ch in s.chars() {
            data.push(ch == '1');
        }
        Ok(Self { data })
    }
}

impl Block {
    pub fn new(data: BitVec) -> Result<Self> {
        if data.len() != 64 {
            // FIXME: Should I keep? The name would change
            return Err(Error::InvalidIterableLength {
                expected: 64,
                got: data.len(),
            });
        }
        Ok(Self { data })
    }

    pub fn to_bitvec(&self) -> BitVec {
        self.data.clone()
    }

    /// returns inner BitVec while consuming Self
    pub fn into_bitvec(&self) -> BitVec {
        let result = self.data.to_owned();
        let _ = self;
        result
    }

    pub fn from_hex_str(s: &str) -> Result<Self> {
        let data_num =
            u64::from_str_radix(s, 16).map_err(|_| Error::StringParseError(s.to_string()))?;
        let s = format!("{data_num:0>width$b}", width = s.len() * 4);
        Self::from_str(s.as_str())
    }

    pub fn to_hex_string(&self) -> String {
        let result = u64::from_str_radix(self.to_string().as_str(), 2).unwrap(); // FIXME: unwrap and blanket implementation
        format!("{:0>16X}", result)
    }

    /// Returns new key with bits shifted according to given scheme. Trims the key if the scheme is shorter
    pub fn shift_scheme(&self, scheme: ShiftSchemes) -> Result<Self> {
        let needed_len = scheme.as_slice().len();
        if self.data.len() < needed_len {
            return Err(Error::InvalidIterableLength {
                expected: needed_len,
                got: self.data.len(),
            });
        }
        let data = scheme.shift(self.into_bitvec())?;
        Self::new(data)
    }

    pub fn encode(&self, key: MainKey) -> Result<Self> {
        // let data = ShiftSchemes::IP.shift(self.as_bitvec())?;
        let data = self.shift_scheme(ShiftSchemes::IP)?.into_bitvec();
        let (left, right) = data.split_at(data.len() / 2);
        let mut left = left.to_bitvec();
        let mut right = right.to_bitvec();
        for round in 1..=16 {
            swap(&mut left, &mut right);
            right ^= self.f(
                left.clone(),
                &key.get_round_key(round, ShiftDirection::Left)?,
            )?;
        }
        swap(&mut left, &mut right);
        left.extend(right);
        // let data = ShiftSchemes::IP1.shift(data)?;
        self.shift_scheme(ShiftSchemes::IP1)
        // Ok(Self { data })
    }

    fn f(&self, right: BitVec, key: &MainKey) -> Result<BitVec> {
        let key = key.to_bitvec();
        let right = ShiftSchemes::E.shift(right)? ^ key;

        let blocks = right.chunks(6).map(|it| it.to_owned());
        let schemes = ShiftSchemes::get_s_schemes();
        let right = blocks
            .zip(schemes)
            .map(|(block, scheme)| {
                let scheme = scheme.as_slice();
                let pos = Self::block_to_pos(block);
                let block: BitVec<usize, bitvec::order::LocalBits> =
                    BitVec::from_element(scheme[pos as usize]);
                block
            })
            .concat();

        ShiftSchemes::P.shift(right)
    }

    /// Returns block's value's position on ShiftSchemes::S(1-8) schemes
    fn block_to_pos(block: BitVec) -> u8 {
        let i_parts = [block[0], block[5]]
            .iter()
            .map(|bit| bit.to_owned() as u16)
            .reduce(|prev, cur| prev * 10 + cur)
            .expect("What?"); // FIXME:
        let j_parts = [block[1], block[2], block[3], block[4]]
            .iter()
            .map(|bit| bit.to_owned() as u16)
            .reduce(|prev, cur| prev * 10 + cur)
            .expect("What?"); // FIXME:
        let i_pos = u16::from_be(i_parts) as u8; // FIXME: from_be or from_str_radix?
        let j_pos = u16::from_be(j_parts) as u8;
        j_pos + i_pos * 16 // a row is 16 nums long hence i_pos * 16
    }
}
