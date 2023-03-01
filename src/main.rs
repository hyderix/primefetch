mod primality;
mod primefetch;

use std::{env,process};

use primality::utils::is_prime;
use primefetch::config::Config;
use primefetch::cli::{print_help,gen_config};
use primality::utils::next_prime;

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
    } else {
        if config.quiet {
            process::exit(1);
        } else {
            println!("{} is NOT PRIME! Next prime is {}", number, next_prime(number));
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

    let number = match config.get_number() {
        Some(num) => num,
        None => 0_u64
    };

    if !config.until_mode {
        check_primality(&config);
    } else {
        check_until(number);
    }
}
