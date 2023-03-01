pub mod config {
    pub struct Config {
        number: Option<u64>,
        pub until_mode: bool,
        pub quiet: bool,
        pub help: bool,
        file_name: Option<String>,
    }
    impl Config {
        pub fn new(
            number: Option<u64>,
            until_mode: bool,
            quiet: bool,
            help: bool,
            file_name: Option<String>,
        ) -> Config {
            Config {
                number,
                until_mode,
                quiet,
                help,
                file_name,
            }
        }

        pub fn get_number(&self) -> Option<u64> {
            self.number
        }
    }
}

pub mod cli {
    pub fn print_help() {
        use std::process;
        print!(
            "primefetch [OPTIONS] [NUMBER|FILE PATH]

        Perform operations and checks on numbers regarding primality.
        
        OPTIONS:
        
        --help, -h - Show this help.
        --quiet, -q - Do not output anything unneccesary.
        --count-to - Count from 2 to NUMBER instead of checking NUMBER for primality.
        --file, -f <FILE> - Read lines from <FILE> and output only the prime numbers.
        \n"
        );

        process::exit(0);
    }

    use crate::primefetch::config::Config;
    pub fn gen_config(args: Vec<String>) -> Config {
        use std::process;

        // Quiet mode - YES or nah
        let mut quiet: bool = false;
        // Count to a number, or check the number - that's the question
        let mut count: bool = false;
        // To help, or to not help
        let mut help: bool = false;
        // File name is optional
        let mut file_name: Option<String> = None;

        for (index, arg) in args.iter().enumerate() {
            if arg == "--quiet" || arg == "-q" {
                quiet = true;
            }

            if arg == "--count-to" {
                count = true;
            }

            if arg == "--help" || arg == "-h" {
                help = true;
            }

            if arg == "--file" || arg == "-f" {
                if index >= args.len() {
                    eprintln!("You must provide a file name!");
                    process::exit(64);
                }
                file_name = Some(args[index + 1].to_string());
                println!("{:?}", &file_name)
            }
        }

        if help {
            // Return config with help
            return Config::new(None, false, false, true, None);
        }

        let number: String = match args.last() {
            Some(a) => a.to_string(),
            None => {
                eprintln!("Args are empty");
                process::exit(64)
            }
        };

        let number: Option<u64> = match number.trim().parse() {
            Ok(num) => Some(num),
            Err(_) => {
                eprintln!("Thou must enter your number last!");
                process::exit(64);
            }
        };

        Config::new(number, count, quiet, help, file_name)
    }
}

pub mod cli_utils {

    use crate::primality::utils::is_prime;

    pub struct PrimesUntil {
        num: u64,
        primes_until: Vec<u64>,
        count: u64,
    }

    impl PrimesUntil {
        pub fn get_count(&self) -> u64 {
            self.count
        }
        pub fn get_num(&self) -> u64 {
            self.num
        }
        pub fn get_primes(&self) -> &Vec<u64> {
            &self.primes_until
        }
    }

    pub fn check_until(num: u64) -> PrimesUntil {
        let mut count: u64 = 0;
        let mut result_vec: Vec<u64> = vec![];
        for i in (1..num).step_by(2) {
            if is_prime(i) {
                result_vec.push(i);
                count += 1;
            }
        }

        PrimesUntil { num , primes_until: result_vec, count }
    }
}
