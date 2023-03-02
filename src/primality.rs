pub mod utils {
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false
        }

        if n % 2 == 0 {
            return false;
        }

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
        let mut counter = number+1;
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
        let mut counter = number-1;
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
}
