use des::{Block, MainKey};
use miette::{IntoDiagnostic, Result as miette_result};
use std::io::stdin;

fn get_input(text: &str) -> miette_result<String> {
    println!("{text}");
    let mut input = String::with_capacity(16);
    stdin().read_line(&mut input).into_diagnostic()?;
    let input = input.trim();
    Ok(input.to_owned())
}

fn main() -> miette_result<()> {
    let key =
        MainKey::from_hex_str(get_input("Input hex main key: ")?.as_str()).into_diagnostic()?;
    let plain_text =
        Block::from_hex_str(get_input("Input plain text")?.as_str()).into_diagnostic()?;
    let plain_text = plain_text.encode(&key).into_diagnostic()?;
    println!("Result: {}", plain_text.to_hex_string());

    Ok(())
}
