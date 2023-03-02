
pub mod config {
    use clap::Parser;
    #[derive(Parser)]
    #[command(author, version)]
    pub struct Config {
        pub number: Option<u64>,
        #[arg(long, value_name = "NUMBER")]
        pub count_to: bool,

        #[arg(long, short)]
        pub quiet: bool,

        // #[arg(short, long)]
        // pub help: bool,

        #[arg(long)]
        pub color: bool,

        #[arg(long, short, value_name = "FILE")]
        pub file_name: Option<String>,
    }
    impl Config {
        // pub fn new(
            // number: Option<u64>,
            // count_to: bool,
            // quiet: bool,
            // help: bool,
            // color: bool,
            // file_name: Option<String>,
        // ) -> Config {
            // Config {
                // number,
                // count_to,
                // quiet,
                // help,
                // color,
                // file_name,
            // }
        // }
// 
        // pub fn get_number(&self) -> Option<u64> {
            // self.number
        // }
    }
}

pub mod cli_utils {

    use crate::primality::utils::{is_prime, next_prime, previous_prime};

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
        result_vec.push(2_u64);
        for i in (3..num).step_by(2) {
            if is_prime(i) {
                result_vec.push(i);
                count += 1;
            }
        }

        PrimesUntil { num , primes_until: result_vec, count }
    }

    use colored::Colorize;
    pub fn format_strings(number: u64, color: bool) -> Vec<String> {
        if color {
            let mut res: Vec<String> = vec![];

            let number_line: String = format!("{} {}", "Number:".bold() , number.to_string().cyan());
            res.push(number_line);

            if is_prime(number) {
                res.push(format!("{} {}", "Primality:".bold(), "PRIME".green().bold()));
            } else {
                res.push(format!("{} {}", "Primality:".bold(), "NOT PRIME".red().bold()));
            }

            res.push(format!("{} {}", "Next prime:".bold(), next_prime(number).to_string().yellow()));
            res.push(format!("{}: {}", "Previous prime".bold(),  match previous_prime(number) {
                Some(num) => num.to_string(),
                None => "None".to_string(),
            }.yellow()));

            res
        } else {
            let mut res: Vec<String> = vec![];

            let number_line: String = format!("Number: {}" , number);
            res.push(number_line);

            if is_prime(number) {
                res.push("Primality: PRIME".to_string());
            } else {
                res.push("Primality: NOT PRIME".to_string());
            }

            res.push(format!("Next prime: {}", next_prime(number)));
            res.push(format!("Previous prime: {}", match previous_prime(number) {
                Some(num) => num.to_string(),
                None => "None available".to_string(),
            }));

            res
        }
    }
}
