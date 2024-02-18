use bitvec::vec::BitVec;
use itertools::Itertools;

use super::{Error, Result};
pub trait FromHexStr: Sized {
    fn from_hex_str(s: &str) -> Result<Self>;
}

pub trait ToUpperHex {
    fn to_upper_hex(&self) -> String;
}

pub trait ToLowerHex {
    fn to_lower_hex(&self) -> String;
}

impl FromHexStr for BitVec {
    fn from_hex_str(s: &str) -> Result<Self> {
        let mut result: Vec<String> = Vec::new();
        let s = s.chars();
        let s = s.into_iter();
        let s = s.chunks(16);
        let s = s.into_iter();
        for chunk in s {
            let chunk = String::from_iter(chunk);
            let chunk = format!("{:0>16}", chunk);

            let key_num = u64::from_str_radix(chunk.as_str(), 16)
                .map_err(|_| Error::StringParseError(chunk.to_string()))?;
            let chunk = format!("{key_num:0>width$b}", width = chunk.len() * 4);
            result.push(chunk);
        }
        let result = result.concat();
        let result = result.as_str();

        let mut data = BitVec::new();
        for ch in result.chars() {
            data.push(ch == '1');
        }
        Ok(data)
    }
}

impl ToUpperHex for BitVec {
    fn to_upper_hex(&self) -> String {
        let s = self.to_string();
        let s = s.trim_matches(['[', ']']).split(", ").collect::<String>();
        let s = s.chars();
        let s = s.chunks(64);
        let s = s.into_iter().map(|chunk| {
            let chunk = String::from_iter(chunk);
            let chunk = u64::from_str_radix(chunk.as_str(), 2).unwrap();
            format!("{chunk:X}")
        });
        String::from_iter(s)
    }
}

impl ToLowerHex for BitVec {
    fn to_lower_hex(&self) -> String {
        let s = self.to_string();
        let s = s.trim_matches(['[', ']']).split(", ").collect::<String>();
        let s = s.chars();
        let s = s.chunks(64);
        let s = s.into_iter().map(|chunk| {
            let chunk = String::from_iter(chunk);
            let chunk = u64::from_str_radix(chunk.as_str(), 2).unwrap();
            format!("{chunk:x}")
        });
        String::from_iter(s)
    }
}
