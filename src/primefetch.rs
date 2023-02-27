pub mod config {
    pub struct Config {
        number: u64,
        pub until_mode: bool,
        pub quiet: bool,
        pub help: bool,
        file_name: String,
    }
    impl Config {
        pub fn new(
            number: u64, until_mode: bool,
            quiet: bool,
            help: bool,
            file_name: Option<String>,
        ) -> Config {
            let file_name: String = match file_name {
                Some(string) => string,
                _ => {
                    todo!()
                }
            };
            Config {
                number,
                until_mode,
                quiet,
                help,
                file_name,
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
        print!(
            "primefetch [OPTIONS] [NUMBER]

        Perform checks on numbers regarding primality.
        
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

        // let mut index = 0;

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
                file_name = Some(args[index+1].to_string());
            }

            // index += 1;
        }

        if help {
            return Config::new(0, false, false, true, None);
        }

        let number: String = match args.last() {
            Some(a) => a.to_string(),
            None => panic!("Args are empty"),
        };

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Thou must enter your number last!");
                process::exit(64);
            }
        };

        Config::new(number, count, quiet, help, file_name)
    }
}
