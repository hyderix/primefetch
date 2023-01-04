pub mod primality;
pub mod primefetch;

use std::{env,process};

use primality::primality::is_prime;
use primefetch::primefetch::Config;
use primefetch::cli::print_help;

fn gen_config(args: Vec<String>) -> Config {

    // Quiet mode - YES or nah
    let mut quiet: bool = false;
    // Count to a number, or check the number - that's the question
    let mut count: bool = false;
    // To help, or to not help
    let mut help: bool = false;
    for arg in &args {
        if arg == "--quiet" || arg == "-q" {
            quiet = true;
        }

        if arg == "--count-to" {
            count = true;
        }

        if arg == "--help" || arg == "-h" {
            help = true;
        }
    }

    if help {
        return Config::new(0, false, false, true);
    }

    let number: String = match args.last() {
        Some(a) => a.to_string(),
        None => panic!("Args are empty"),
    };

    let number: u64 = match number.trim().parse() {
        Ok( num ) => num,
        Err(_) => {
            eprintln!("Thou must enter your number last!");
            process::exit(64);
        }
    };

    Config::new(number, count, quiet, help)

}

fn check_primality(config: &Config) {
    if is_prime(config.get_number()) {
        if !config.quiet {
            println!("{} is PRIME!", config.get_number());
        }
        process::exit(0);
    } else {
        if config.quiet {
            process::exit(1);
        } else {
            println!("{} is NOT PRIME!", config.get_number());
            process::exit(0);
        }
    }
}

fn check_until(number: u64) {
    let mut count = 0;
    for i in 2..number {
        if is_prime(i) {
            println!("{}", i);
            count += 1;
        }
    }

    eprintln!("{} primes found until {}", count, number);
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

    if !config.until_mode {

        check_primality(&config);

    } else {
        
        check_until(config.get_number());

    }
}
