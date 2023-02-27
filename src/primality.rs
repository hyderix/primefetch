pub mod utils {
    pub fn is_prime(n: u64) -> bool {
        let mut a = 1;
        let largest = loop {
            if a*a > n {
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

}
