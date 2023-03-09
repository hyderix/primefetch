mod primality;
mod primefetch;

use std::process::exit;

use primefetch::config::Config;

use clap::Parser;

use primefetch::cli_utils::primefetch;

fn main() {
    let config = Config::parse();

    match primefetch(config) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("An error occurred in program");
            exit(1);
        },
    }

}
