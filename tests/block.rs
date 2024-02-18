use bitvec::prelude::*;
use des::{Block, FromHexStr, MainKey, Result, ToUpperHex};
use std::str::FromStr;

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
    assert_eq!(block.to_upper_hex(), "0F3CA59D512CA5C6");

    let block = Block::new(BitVec::from(
        bits![usize, bitvec::order::LocalBits; 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 0, 1],
    ))?;
    assert_eq!(block.to_upper_hex(), "9C36A6CA96990F95");
    Ok(())
}

#[test]
fn test_encode() -> Result<()> {
    let plain_text = Block::from_hex_str("8787878787878787")?;
    assert_eq!(plain_text.as_bitvec().len(), 64);
    let key = MainKey::from_hex_str("0E329232EA6D0D73")?;
    assert_eq!(plain_text.encode(&key)?.to_upper_hex(), "0000000000000000");
    Ok(())
}

#[test]
fn test_decode() -> Result<()> {
    let cipher_text = Block::from_hex_str("0000000000000000")?;
    let key = MainKey::from_hex_str("0E329232EA6D0D73")?;
    assert_eq!(cipher_text.decode(&key)?.to_upper_hex(), "8787878787878787");
    Ok(())
}
