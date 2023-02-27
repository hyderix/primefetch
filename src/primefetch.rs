
pub mod config {
    pub struct Config {
        number: u64,
        pub until_mode: bool,
        pub quiet: bool,
        pub help: bool
    }

    impl Config {
        pub fn new(num: u64, count_until: bool, shush: bool, help_val: bool) -> Config {
            Config {
                number: num,
                until_mode: count_until,
                quiet: shush,
                help: help_val,
            }
        }

        pub fn get_number(&self) -> u64 {
           self.number 
        }
    }
}

pub mod cli {
    pub fn print_help() {
        use std::process;
        print!("primefetch [OPTIONS] [NUMBER]

        Perform checks on numbers regarding primality.
        
        OPTIONS:
        
        --help, -h - Show this help.
        --quiet, -q - Do not output anything unneccesary.
        --count-to - Count from 2 to NUMBER instead of checking NUMBER for primality.
        --file, -f <FILE> - Read lines from <FILE> and output only the prime numbers.
        \n");

        process::exit(0);
    }

    pub fn gen_config(args: Vec<String>) -> Config {
        use std::process;

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
            },
        };

        Config::new(number, count, quiet, help)

    }
}
