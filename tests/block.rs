use bitvec::prelude::*;
use des::{Block, MainKey, Result};
use std::str::FromStr;

// Begin part yanked from main_block tests
#[test]
fn test_from_string() -> Result<()> {
    let block = Block::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0],
    ))?;
    assert_eq!(
        Block::from_str("0000010100011001101011111000100111001010110111111111010010111110")?,
        block
    );
    Ok(())
}

#[test]
fn test_from_hex_string() -> Result<()> {
    let block = Block::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0],
    ))?;
    assert_eq!(Block::from_hex_str("0F3CA59D512CA5C6")?, block);
    Ok(())
}

#[test]
fn test_to_string() -> Result<()> {
    let block = Block::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0],
    ))?;
    assert_eq!(
        block.to_string(),
        "0000111100111100101001011001110101010001001011001010010111000110"
    );
    Ok(())
}

#[test]
fn test_to_hex_string() -> Result<()> {
    let block = Block::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0],
    ))?;
    assert_eq!(block.to_hex_string(), "0F3CA59D512CA5C6");

    let block = Block::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1],
    ))?;
    assert_eq!(block.to_hex_string(), "9C36A6CA96990F95");
    Ok(())
}
// End part yanked from main_block tests

#[test]
fn test_encode() -> Result<()> {
    let plain_text = Block::from_hex_str("8787878787878787")?;
    assert_eq!(plain_text.to_bitvec().len(), 64);
    let key = MainKey::from_hex_str("0E329232EA6D0D73")?;
    assert_eq!(plain_text.encode(&key)?.to_hex_string(), "0000000000000000");
    Ok(())
}

#[test]
fn test_decode() -> Result<()> {
    let cipher_text = Block::from_hex_str("0000000000000000")?;
    let key = MainKey::from_hex_str("0E329232EA6D0D73")?;
    assert_eq!(
        cipher_text.decode(&key)?.to_hex_string(),
        "8787878787878787"
    );
    Ok(())
}
