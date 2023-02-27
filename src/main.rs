mod primality;
mod primefetch;

use std::{env,process};

use primality::utils::is_prime;
use primefetch::config::Config;
use primefetch::cli::{print_help,gen_config};

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
