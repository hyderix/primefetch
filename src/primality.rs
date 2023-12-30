pub mod utils {
    pub fn is_prime(n: u64) -> bool {
        if n == 1 || n == 0 {
            return false;
        }

        if n % 2 == 0 && n != 2 {
            return false;
        }

        let largest_num_tested: u64 = integer_square_root(n);

        for num in 2..largest_num_tested {
            if n % num == 0 {
                return false;
            }
        }

        true
    }

    fn integer_square_root(n: u64) -> u64 {
        let mut a = 1;
        let largest = loop {
            if a * a > n {
                break a;
            } else {
                a += 1;
            }
        };

        largest
    }

    pub fn next_prime(number: u64) -> u64 {
        let mut counter = number + 1;
        loop {
            if is_prime(counter) {
                break counter;
            }
            counter += 1;
        }
    }

    pub fn previous_prime(number: u64) -> Option<u64> {
        if number < 2 {
            return None;
        }
        let mut counter = number - 1;
        loop {
            if is_prime(counter) {
                break Some(counter);
            }
            counter -= 1;
        }
    }

    pub fn prime_factors(number: u64) -> Vec<u64> {
        let largest_num_tested = integer_square_root(number) + 1;
        let mut res_vector: Vec<u64> = vec![];

        for num in 2..=largest_num_tested {
            if number % num == 0 {
                res_vector.push(num);
            }
        }

        res_vector
    }
}

#[cfg(test)]
mod test {
    use crate::primality::utils::{is_prime, next_prime, previous_prime, prime_factors};

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(45), false);
        assert_eq!(is_prime(18809394909), false);
        assert_eq!(is_prime(18809394911), true);
        assert_eq!(is_prime(0), false);
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(18), 19);
        assert_eq!(next_prime(17), 19);
        assert_eq!(next_prime(18809394909), 18809394911);
        assert_eq!(next_prime(0), 2);
    }

    #[test]
    fn test_prev_prime() {
        assert_eq!(previous_prime(20), Some(19));
        assert_eq!(previous_prime(3), Some(2));
        assert_eq!(previous_prime(1), None);
    }

    #[test]
    fn test_prime_factors() {
        let factor_six = prime_factors(6);
        let factor_fifteen = prime_factors(15);
        let factor_large = prime_factors(149805493);
        assert_eq!(factor_six, vec![2, 3]);
        assert_eq!(factor_fifteen, vec![3, 5]);
        assert_eq!(factor_large, vec![1789, 83737]);
    }
}
