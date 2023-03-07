mod primality;
mod primefetch;

use primefetch::config::Config;

use clap::Parser;

use std::io::Result;

use primefetch::cli_utils::primefetch;

fn main() -> Result<()> {
    let config = Config::parse();

    primefetch(config);

    Ok(())
}
