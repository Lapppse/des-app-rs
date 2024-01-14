use des::{Block, Result, ShiftSchemes};
use std::str::FromStr;

#[test]
fn test_ip() -> Result<()> {
    let block =
        Block::from_str("0000000100100011010001010110011110001001101010111100110111101111")?;
    let block = ShiftSchemes::IP.shift(block.to_bitvec())?;
    assert_eq!(
        Block::new(block)?,
        Block::from_str("1100110000000000110011001111111111110000101010101111000010101010")?
    );

    Ok(())
}

#[test]
fn test_ip1() -> Result<()> {
    let block =
        Block::from_str("0000101001001100110110011001010101000011010000100011001000110100")?;
    let block = ShiftSchemes::IP1.shift(block.to_bitvec())?;
    assert_eq!(
        Block::new(block)?,
        Block::from_str("1000010111101000000100110101010000001111000010101011010000000101")?
    );

    Ok(())
}
