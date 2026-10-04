/// Cipolla algorithm
///
/// Solving quadratic residue problem:
///     x^2 = a (mod p) , p is an odd prime
/// with O(M*log(n)) time complexity, M depends on the complexity of complex numbers multiplication.
///
/// Wikipedia reference: https://en.wikipedia.org/wiki/Cipolla%27s_algorithm
/// When a is the primitive root modulo n, the answer is unique.
/// Otherwise it will return the smallest positive solution
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use rand::RngExt;

use super::{fast_power, PCG32};

#[derive(Debug)]
struct CustomFiniteField {
    modulus: u64,
    i_square: u64,
}

impl CustomFiniteField {
    pub fn new(modulus: u64, i_square: u64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
}

#[derive(Clone, Debug)]
struct CustomComplexNumber {
    real: u64,
    imag: u64,
    f: Rc<CustomFiniteField>,
}

impl CustomComplexNumber {
    pub fn new(real: u64, imag: u64, f: Rc<CustomFiniteField>) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}

    pub fn mult_other(&mut self, rhs: &Self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mult_other)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mult_other");
}

    pub fn mult_self(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mult_self)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mult_self");
}

    pub fn fast_power(mut base: Self, mut power: u64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fast_power)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fast_power");
}
}

fn is_residue(x: u64, modulus: u64) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_residue)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_residue");
}

/// The Legendre symbol `(a | p)`
///
/// Returns 0 if a = 0 mod p, 1 if a is a square mod p, -1 if it not a square mod p.
///
/// <https://en.wikipedia.org/wiki/Legendre_symbol>
pub fn legendre_symbol(a: u64, odd_prime: u64) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (legendre_symbol)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement legendre_symbol");
}

// return two solutions (x1, x2) for Quadratic Residue problem x^2 = a (mod p), where p is an odd prime
// if a is Quadratic Nonresidues, return None
pub fn cipolla(a: u32, p: u32, seed: Option<u64>) -> Option<(u32, u32)> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cipolla)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cipolla");
}

/// Returns one of the two possible solutions of _x² = a mod p_, if any.
///
/// The other solution is _-x mod p_. If there is no solution, returns `None`.
///
/// Reference: H. Cohen, _A course in computational algebraic number theory_, Algorithm 1.4.3
///
/// ## Implementation details
///
/// To avoid multiplication overflows, internally the algorithm uses the `128`-bit arithmetic.
///
/// Also see [`cipolla`].
pub fn tonelli_shanks(a: i64, odd_prime: u64) -> Option<u64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (tonelli_shanks)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement tonelli_shanks");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn tonelli_shanks_residues(x: u64, odd_prime: u64) -> Option<(u64, u64)> {
        let x = tonelli_shanks(x as i64, odd_prime)?;
        let x2 = (-(x as i64)).rem_euclid(odd_prime as i64) as u64;
        Some(if x < x2 { (x, x2) } else { (x2, x) })
    }

    #[test]
    fn cipolla_small_numbers() {
        assert_eq!(cipolla(1, 43, None), Some((1, 42)));
        assert_eq!(cipolla(2, 23, None), Some((5, 18)));
        assert_eq!(cipolla(17, 83, Some(42)), Some((10, 73)));
    }

    #[test]
    fn tonelli_shanks_small_numbers() {
        assert_eq!(tonelli_shanks_residues(1, 43).unwrap(), (1, 42));
        assert_eq!(tonelli_shanks_residues(2, 23).unwrap(), (5, 18));
        assert_eq!(tonelli_shanks_residues(17, 83).unwrap(), (10, 73));
    }

    #[test]
    fn cipolla_random_numbers() {
        assert_eq!(cipolla(392203, 852167, None), Some((413252, 438915)));
        assert_eq!(
            cipolla(379606557, 425172197, None),
            Some((143417827, 281754370))
        );
        assert_eq!(
            cipolla(585251669, 892950901, None),
            Some((192354555, 700596346))
        );
        assert_eq!(
            cipolla(404690348, 430183399, Some(19260817)),
            Some((57227138, 372956261))
        );
        assert_eq!(
            cipolla(210205747, 625380647, Some(998244353)),
            Some((76810367, 548570280))
        );
    }

    #[test]
    fn tonelli_shanks_random_numbers() {
        assert_eq!(
            tonelli_shanks_residues(392203, 852167),
            Some((413252, 438915))
        );
        assert_eq!(
            tonelli_shanks_residues(379606557, 425172197),
            Some((143417827, 281754370))
        );
        assert_eq!(
            tonelli_shanks_residues(585251669, 892950901),
            Some((192354555, 700596346))
        );
        assert_eq!(
            tonelli_shanks_residues(404690348, 430183399),
            Some((57227138, 372956261))
        );
        assert_eq!(
            tonelli_shanks_residues(210205747, 625380647),
            Some((76810367, 548570280))
        );
    }

    #[test]
    fn no_answer() {
        assert_eq!(cipolla(650927, 852167, None), None);
        assert_eq!(tonelli_shanks(650927, 852167), None);
    }
}
