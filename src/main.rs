mod primality;
mod primefetch;

use std::{env, process};

use primality::utils::is_prime;
use primality::utils::next_prime;
use primefetch::cli::{gen_config, print_help};
use primefetch::cli_utils::check_until;
use primefetch::config::Config;

fn check_primality(config: &Config) {
    let number = match config.get_number() {
        Some(num) => num,
        None => {
            eprintln!("An error occured, no number in Config struct");
            process::exit(64);
        }
    };
    if is_prime(number) {
        if !config.quiet {
            println!("{} is PRIME!", number);
        }
        process::exit(0);
    } else if config.quiet {
        process::exit(1);
    } else {
        println!(
            "{} is NOT PRIME! Next prime is {}",
            number,
            next_prime(number)
        );
        process::exit(0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Too few arguments!");
        process::exit(64);
    }

    let config = gen_config(args);

    if config.help {
        print_help();
    }

    let number: u64 = config.get_number().unwrap_or(0_u64);

    if !config.until_mode {
        check_primality(&config);
    } else {
        let result = check_until(number);
        for res in result.get_primes().iter() {
            println!("{}", res);
        }
        eprintln!("{} primes found until {}.", result.get_count(), result.get_num());
    }
}
