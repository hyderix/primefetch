pub mod utils {
    pub fn is_prime(n: u64) -> bool {
        let mut a = 1;
        let largest = loop {
            if a * a > n {
                break a;
            } else {
                a += 1;
            }
        };

        let largest_num_tested: u64 = largest;

        for num in 2..largest_num_tested {
            if n % num == 0 {
                return false;
            }
        }

        true
    }

    pub fn next_prime(number: u64) -> u64 {
        let mut counter = number;
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
        let mut counter = number;
        loop {
            if is_prime(counter) {
                break Some(counter);
            }
            counter -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::primality::utils::{is_prime, next_prime, previous_prime};

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(45), false);
        assert_eq!(is_prime(18809394909), false);
        assert_eq!(is_prime(18809394911), true);
    }

    #[test]
    fn test_next_prime() {
        assert_eq!(next_prime(18), 19);
        assert_eq!(next_prime(17), 17);
        assert_eq!(next_prime(18809394909), 18809394911);
    }

    #[test]
    fn test_prev_prime() {
        assert_eq!(previous_prime(20), Some(19));
        assert_eq!(previous_prime(3), Some(3));
        assert_eq!(previous_prime(1), None);
    }
}
