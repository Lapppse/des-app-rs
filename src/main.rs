use des::{Block, MainKey};
use miette::{IntoDiagnostic, Result as m_result};
use std::str::FromStr;

fn main() -> m_result<()> {
    let plain_text = Block::from_str("8787878787878787").into_diagnostic()?;
    let key = MainKey::from_str("0E329232EA6D0D73").into_diagnostic()?;
    println!("{}", plain_text.encode(key).into_diagnostic()?);

    Ok(())
}
