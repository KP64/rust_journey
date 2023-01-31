use std::str::FromStr;

use anyhow::Result;
use idiomatic_rust::Money;

fn main() -> Result<()> {
    let money = Money::from_str("200.23 $")?;
    println!("{money}");
    Ok(())
}
