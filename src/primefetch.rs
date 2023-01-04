
pub mod primefetch {
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
        \n");

        process::exit(0);
    }
}
