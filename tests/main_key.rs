use bitvec::prelude::*;
use des::{MainKey, Result, ShiftSchemes};
use std::str::FromStr;

#[test]
fn test_from_string() -> Result<()> {
    let key = MainKey::new(BitVec::from(
        bits![u8, bitvec::order::LocalBits; 0, 1, 0, 1, 0, 1],
    ));
    assert_eq!(MainKey::from_str("010101")?, key);
    Ok(())
}

#[test]
fn test_from_hex_string() -> Result<()> {
    let key = MainKey::new(BitVec::from(
        bits![u8, bitvec::order::LocalBits; 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
    ));
    assert_eq!(MainKey::from_str("FE5")?, key);

    let key = MainKey::new(BitVec::from(
        bits![u8, bitvec::order::LocalBits; 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    ));
    assert_eq!(MainKey::from_str("878067467E19F5")?, key);

    let key = MainKey::from_str("33F0CFAC3C033A")?;
    assert_eq!(
        key.to_string(),
        "00110011111100001100111110101100001111000000001100111010"
    );
    Ok(())
}

#[test]
fn test_to_string() -> Result<()> {
    let key = BitVec::from(bits![u8, bitvec::order::LocalBits; 0, 0, 1, 0, 1, 0]);
    let key = MainKey::new(key);
    assert_eq!(key.to_string(), "001010");

    let key = MainKey::new(BitVec::from(
        bits![u8, bitvec::order::LocalBits; 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
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
        bits![u8, bitvec::order::LocalBits; 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0,1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0],
    ));
    assert_eq!(key.to_hex_string(), "C3C033A33F0CFA");

    let key = MainKey::new(BitVec::from(
        bits![u8, bitvec::order::LocalBits; 1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1],
    ));
    assert_eq!(key.to_hex_string(), "878067467E19F5");
    Ok(())
}

#[test]
fn test_round_shift() -> Result<()> {
    let key = MainKey::from_str("AABB09182736CCDD")
        .and_then(|key| key.shift_scheme(ShiftSchemes::PC1))
        .and_then(|key| key.shift_round(1))?; // FIXME: as_slice and as_bitvec
    let should_be = MainKey::from_str("878067567E19F4")?;
    assert_eq!(key.to_hex_string(), should_be.to_hex_string());

    let key = MainKey::from_str("AABB09182736CCDD")
        .and_then(|key| key.shift_scheme(ShiftSchemes::PC1))
        .and_then(|key| key.shift_round(16))?;
    let should_be = MainKey::from_str("C3C033A33F0CFA")?;
    assert_eq!(key.to_hex_string(), should_be.to_hex_string());
    Ok(())
}

#[test]
fn test_pc1_shift() -> Result<()> {
    let key = MainKey::from_str("AABB09182736CCDD")?.shift_scheme(ShiftSchemes::PC1)?;
    assert_eq!(key.to_hex_string(), "C3C033A33F0CFA");
    Ok(())
}

#[test]
fn test_round_key() -> Result<()> {
    let key = MainKey::from_str("AABB09182736CCDD").and_then(|key| key.get_round_key(1))?;
    let should_be = MainKey::from_str("194CD072DE8C")?; // FIXME?
    assert_eq!(key.to_hex_string(), should_be.to_hex_string());

    let key = MainKey::from_str("AABB09182736CCDD").and_then(|key| key.get_round_key(16))?;
    let should_be = MainKey::from_str("181C5D75C66D")?;
    assert_eq!(key.to_hex_string(), should_be.to_hex_string());
    Ok(())
}
