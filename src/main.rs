mod primality;
mod primefetch;

use std::{env, process};

use primefetch::cli::{gen_config, print_help, check_primality};
use primefetch::cli_utils::{check_until, format_strings};


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
        // check_primality(&config);
        
        for string in format_strings(number, config.color).iter() {
            println!("{}", string);
        }
    } else {
        let result = check_until(number);
        for res in result.get_primes().iter() {
            println!("{}", res);
        }
        eprintln!("{} primes found until {}.", result.get_count(), result.get_num());
    }
}
