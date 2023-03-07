mod primality;
mod primefetch;

use primefetch::cli_utils::{check_until, format_strings};
use primefetch::config::Config;

use clap::Parser;

fn main() {
    let config = Config::parse();

    let number: u64 = config.number.expect("No number provided");

    if !config.count_to {
        for string in format_strings(number, config.color).iter() {
            println!("{}", string);
        }
    } else {
        let result = check_until(number);
        for res in result.get_primes().iter() {
            println!("{}", res);
        }
        eprintln!(
            "{} primes found until {}.",
            result.get_count(),
            result.get_num()
        );
    }
}
