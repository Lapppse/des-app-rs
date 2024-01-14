use bitvec::prelude::*;
use des::{MainKey, Result, ShiftDirection, ShiftSchemes};
use std::str::FromStr;

#[test]
fn test_from_string() -> Result<()> {
    let key = MainKey::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 0, 1, 0, 1, 0, 1],
    ));
    assert_eq!(MainKey::from_str("010101")?, key);
    Ok(())
}

#[test]
fn test_from_hex_string() -> Result<()> {
    let key = MainKey::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
    ));
    assert_eq!(MainKey::from_hex_str("FE5")?, key);

    let key = MainKey::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    ));
    assert_eq!(MainKey::from_hex_str("878067467E19F5")?, key);

    let key = MainKey::from_hex_str("0F3CA59D512CA5C6")?;
    assert_eq!(
        key.to_string(),
        "0000111100111100101001011001110101010001001011001010010111000110"
    );

    let key = MainKey::from_hex_str("33F0CFAC3C033A")?;
    assert_eq!(
        key.to_string(),
        "00110011111100001100111110101100001111000000001100111010"
    );
    Ok(())
}

#[test]
fn test_to_string() -> Result<()> {
    let key = BitVec::from(bits![usize, bitvec::order::LocalBits; 0, 0, 1, 0, 1, 0]);
    let key = MainKey::new(key);
    assert_eq!(key.to_string(), "001010");

    let key = MainKey::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    ));
    assert_eq!(
        key.to_string(),
        "10000111100000000110011101000110011111100001100111110101"
    );
    Ok(())
}

#[test]
fn test_to_hex_string() -> Result<()> {
    let key = MainKey::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0,1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0],
    ));
    assert_eq!(key.to_hex_string(), "C3C033A33F0CFA");

    let key = MainKey::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0],
    ));
    assert_eq!(key.to_hex_string(), "0F3CA59D512CA5C6");

    let key = MainKey::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    ));
    assert_eq!(key.to_hex_string(), "878067467E19F5");
    Ok(())
}

#[test]
fn test_round_shift() -> Result<()> {
    let key = MainKey::from_hex_str("AABB09182736CCDD")
        .and_then(|key| key.shift_scheme(ShiftSchemes::PC1))?;
    let left_shift = key.clone().shift_round(1, ShiftDirection::Left)?;
    let right_shift = key.clone().shift_round(16, ShiftDirection::Right)?;
    assert_eq!(left_shift, MainKey::from_hex_str("878067567E19F4")?);
    assert_eq!(left_shift, right_shift);

    let key = MainKey::from_hex_str("AABB09182736CCDD")
        .and_then(|key| key.shift_scheme(ShiftSchemes::PC1))?;
    let left_shift = key.clone().shift_round(16, ShiftDirection::Left)?;
    let right_shift = key.clone().shift_round(1, ShiftDirection::Right)?;
    assert_eq!(left_shift, MainKey::from_hex_str("C3C033A33F0CFA")?);
    assert_eq!(left_shift, right_shift);
    Ok(())
}

#[test]
fn test_pc1_shift() -> Result<()> {
    let key = MainKey::from_hex_str("AABB09182736CCDD")?.shift_scheme(ShiftSchemes::PC1)?;
    assert_eq!(key, MainKey::from_hex_str("C3C033A33F0CFA")?);
    Ok(())
}

#[test]
fn test_round_key() -> Result<()> {
    let key = MainKey::from_hex_str("AABB09182736CCDD")
        .and_then(|key| key.get_round_key(1, ShiftDirection::Left))?;
    assert_eq!(key, MainKey::from_hex_str("194CD072DE8C")?);

    let key = MainKey::from_hex_str("AABB09182736CCDD")
        .and_then(|key| key.get_round_key(16, ShiftDirection::Left))?;
    assert_eq!(key, MainKey::from_hex_str("181C5D75C66D")?);
    Ok(())
}
