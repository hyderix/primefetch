mod primality;
mod primefetch;

use primefetch::cli_utils::{check_until, format_strings};
use primefetch::config::Config;

use clap::Parser;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    // eprintln!("Too few arguments!");
    // process::exit(64);
    // }
    //
    // let config = gen_config(args);
    //
    // if config.help {
    // print_help();
    // }

    let config = Config::parse();

    let number: u64 = config.number.unwrap_or(0);

    if !config.count_to {
        // check_primality(&config);

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
