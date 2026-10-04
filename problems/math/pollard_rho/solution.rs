use super::miller_rabin;

struct LinearCongruenceGenerator {
    // modulus as 2 ^ 32
    multiplier: u32,
    increment: u32,
    state: u32,
}

impl LinearCongruenceGenerator {
    fn new(multiplier: u32, increment: u32, state: u32) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    fn next(&mut self) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
    fn get_64bits(&mut self) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_64bits)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_64bits");
}
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gcd)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gcd");
}

#[inline]
fn advance(x: u128, c: u64, number: u64) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (advance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement advance");
}

fn pollard_rho_customizable(
    number: u64,
    x0: u64,
    c: u64,
    iterations_before_check: u32,
    iterations_cutoff: u32,
) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pollard_rho_customizable)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pollard_rho_customizable");
}

/*
Note: using this function with `check_is_prime` = false
and a prime number will result in an infinite loop.

RNG's internal state is represented as `seed`. It is
advisable (but not mandatory) to reuse the saved seed value
In subsequent calls to this function.
 */
pub fn pollard_rho_get_one_factor(number: u64, seed: &mut u32, check_is_prime: bool) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pollard_rho_get_one_factor)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pollard_rho_get_one_factor");
}

fn get_small_factors(mut number: u64, primes: &[usize]) -> (u64, Vec<u64>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_small_factors)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_small_factors");
}

fn factor_using_mpf(mut number: usize, mpf: &[usize]) -> Vec<u64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (factor_using_mpf)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement factor_using_mpf");
}

/*
`primes` and `minimum_prime_factors` use usize because so does
LinearSieve implementation in this repository
 */
pub fn pollard_rho_factorize(
    mut number: u64,
    seed: &mut u32,
    primes: &[usize],
    minimum_prime_factors: &[usize],
) -> Vec<u64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (pollard_rho_factorize)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement pollard_rho_factorize");
}


#[cfg(test)]
mod test {
    use super::super::LinearSieve;
    use super::*;

    fn check_is_proper_factor(number: u64, factor: u64) -> bool {
        factor > 1 && factor < number && number.is_multiple_of(factor)
    }

    fn check_factorization(number: u64, factors: &[u64]) -> bool {
        let bases = vec![2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
        let mut prod = 1_u64;
        let mut prime_check = 0_u64;
        for p in factors {
            prod *= *p;
            prime_check |= miller_rabin(*p, &bases);
        }
        prime_check == 0 && prod == number
    }

    #[test]
    fn one_factor() {
        // a few small cases
        let mut sieve = LinearSieve::new();
        sieve.prepare(1e5 as usize).unwrap();
        let numbers = vec![1235, 239874233, 4353234, 456456, 120983];
        let mut seed = 314159_u32; // first digits of pi; nothing up my sleeve
        for num in numbers {
            let factor = pollard_rho_get_one_factor(num, &mut seed, true);
            assert!(check_is_proper_factor(num, factor));
            let factor = pollard_rho_get_one_factor(num, &mut seed, false);
            assert!(check_is_proper_factor(num, factor));
            assert!(check_factorization(
                num,
                &pollard_rho_factorize(num, &mut seed, &sieve.primes, &sieve.minimum_prime_factor)
            ));
        }
        // check if it goes into infinite loop if `number` is prime
        let numbers = vec![
            2, 3, 5, 7, 11, 13, 101, 998244353, 1000000007, 1000000009, 1671398671, 1652465729,
            1894404511, 1683402997, 1661963047, 1946039987, 2071566551, 1867816303, 1952199377,
            1622379469, 1739317499, 1775433631, 1994828917, 1818930719, 1672996277,
        ];
        for num in numbers {
            assert_eq!(pollard_rho_get_one_factor(num, &mut seed, true), num);
            assert!(check_factorization(
                num,
                &pollard_rho_factorize(num, &mut seed, &sieve.primes, &sieve.minimum_prime_factor)
            ));
        }
    }
    #[test]
    fn big_numbers() {
        // Bigger cases:
        // Each of these numbers is a product of two 31 bit primes
        // This shouldn't take more than a 10ms per number on a modern PC
        let mut seed = 314159_u32; // first digits of pi; nothing up my sleeve
        let numbers: Vec<u64> = vec![
            2761929023323646159,
            3189046231347719467,
            3234246546378360389,
            3869305776707280953,
            3167208188639390813,
            3088042782711408869,
            3628455596280801323,
            2953787574901819241,
            3909561575378030219,
            4357328471891213977,
            2824368080144930999,
            3348680054093203003,
            2704267100962222513,
            2916169237307181179,
            3669851121098875703,
        ];
        for num in numbers {
            assert!(check_factorization(
                num,
                &pollard_rho_factorize(num, &mut seed, &[], &[])
            ));
        }
    }
}
