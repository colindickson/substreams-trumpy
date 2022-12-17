use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("erc721", "abis/erc721.json")?
        .generate()?
        .write_to_file("src/abi/erc721.rs")?;
    Abigen::new("factory", "abis/factory.json")?
        .generate()?
        .write_to_file("src/abi/factory.rs")?;

    Ok(())
}
