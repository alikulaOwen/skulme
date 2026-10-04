use core::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A field
///
/// <https://en.wikipedia.org/wiki/Field_(mathematics)>
pub trait Field:
    Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Eq
    + Copy
    + fmt::Debug
{
    const CHARACTERISTIC: u64;
    const ZERO: Self;
    const ONE: Self;

    /// Multiplicative inverse
    fn inverse(self) -> Self;

    /// Z-mod structure
    fn integer_mul(self, a: i64) -> Self;
    fn from_integer(a: i64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from_integer)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from_integer");
}

    /// Iterate over all elements in this field
    ///
    /// The iterator finishes only for finite fields.
    type ElementsIter: Iterator<Item = Self>;
    fn elements() -> Self::ElementsIter;
}

/// Prime field of order `P`, that is, finite field `GF(P) = ℤ/Pℤ`
///
/// Only primes `P` <= 2^63 - 25 are supported, because the field elements are represented by `i64`.
// TODO: Extend field implementation for any prime `P` by e.g. using u32 blocks.
#[derive(Clone, Copy)]
pub struct PrimeField<const P: u64> {
    a: i64,
}

impl<const P: u64> PrimeField<P> {
    /// Reduces the representation into the range [0, p)
    fn reduce(self) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (reduce)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement reduce");
}

    /// Returns the positive integer in the range [0, p) representing this element
    pub fn to_integer(&self) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (to_integer)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement to_integer");
}
}

impl<const P: u64> From<i64> for PrimeField<P> {
    fn from(a: i64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (from)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement from");
}
}

impl<const P: u64> PartialEq for PrimeField<P> {
    fn eq(&self, other: &Self) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (eq)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement eq");
}
}

impl<const P: u64> Eq for PrimeField<P> {}

impl<const P: u64> Neg for PrimeField<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (neg)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement neg");
}
}

impl<const P: u64> Add for PrimeField<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add");
}
}

impl<const P: u64> Sub for PrimeField<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (sub)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement sub");
}
}

impl<const P: u64> Mul for PrimeField<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mul)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mul");
}
}

impl<const P: u64> Div for PrimeField<P> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (div)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement div");
}
}

impl<const P: u64> fmt::Debug for PrimeField<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fmt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fmt");
}
}

impl<const P: u64> Field for PrimeField<P> {
    const CHARACTERISTIC: u64 = P;
    const ZERO: Self = Self { a: 0 };
    const ONE: Self = Self { a: 1 };

    fn inverse(self) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (inverse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement inverse");
}

    fn integer_mul(self, mut n: i64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (integer_mul)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement integer_mul");
}

    type ElementsIter = PrimeFieldElementsIter<P>;

    fn elements() -> Self::ElementsIter {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (elements)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement elements");
}
}

#[derive(Default)]
pub struct PrimeFieldElementsIter<const P: u64> {
    x: i64,
}

impl<const P: u64> Iterator for PrimeFieldElementsIter<P> {
    type Item = PrimeField<P>;

    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}

impl<const P: u64> Hash for PrimeField<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (hash)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement hash");
}
}

// TODO: should we use extended_euclidean_algorithm adjusted to i64?
fn mod_inverse(mut a: i64, mut b: i64) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mod_inverse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mod_inverse");
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_field_elements() {
        fn test<const P: u64>() {
            let expected: HashSet<PrimeField<P>> = (0..P as i64).map(Into::into).collect();
            for gen in 1..P - 1 {
                // every field element != 0 generates the whole field additively
                let gen = PrimeField::from(gen as i64);
                let mut generated: HashSet<PrimeField<P>> = std::iter::once(gen).collect();
                let mut x = gen;
                for _ in 0..P {
                    x = x + gen;
                    generated.insert(x);
                }
                assert_eq!(generated, expected);
            }
        }
        test::<5>();
        test::<7>();
        test::<11>();
        test::<13>();
        test::<17>();
        test::<19>();
        test::<23>();
        test::<71>();
        test::<101>();
    }

    #[test]
    fn large_prime_field() {
        const P: u64 = 2_u64.pow(63) - 25; // largest prime fitting into i64
        type F = PrimeField<P>;
        let x = F::from(P as i64 - 1);
        let y = x.inverse();
        assert_eq!(x * y, F::ONE);
    }

    #[test]
    fn inverse() {
        fn test<const P: u64>() {
            for x in -7..7 {
                let x = PrimeField::<P>::from(x);
                if x != PrimeField::ZERO {
                    // multiplicative
                    assert_eq!(x.inverse() * x, PrimeField::ONE);
                    assert_eq!(x * x.inverse(), PrimeField::ONE);
                    assert_eq!((x.inverse().a * x.a).rem_euclid(P as i64), 1);
                    assert_eq!(x / x, PrimeField::ONE);
                }
                // additive
                assert_eq!(x + (-x), PrimeField::ZERO);
                assert_eq!((-x) + x, PrimeField::ZERO);
                assert_eq!(x - x, PrimeField::ZERO);
            }
        }
        test::<5>();
        test::<7>();
        test::<11>();
        test::<13>();
        test::<17>();
        test::<19>();
        test::<23>();
        test::<71>();
        test::<101>();
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(-6, 7), 1);
        assert_eq!(mod_inverse(-5, 7), -3);
        assert_eq!(mod_inverse(-4, 7), -2);
        assert_eq!(mod_inverse(-3, 7), 2);
        assert_eq!(mod_inverse(-2, 7), 3);
        assert_eq!(mod_inverse(-1, 7), -1);
        assert_eq!(mod_inverse(1, 7), 1);
        assert_eq!(mod_inverse(2, 7), -3);
        assert_eq!(mod_inverse(3, 7), -2);
        assert_eq!(mod_inverse(4, 7), 2);
        assert_eq!(mod_inverse(5, 7), 3);
        assert_eq!(mod_inverse(6, 7), -1);
    }

    #[test]
    fn integer_mul() {
        type F = PrimeField<23>;
        for x in 0..23 {
            let x = F { a: x };
            for n in -7..7 {
                assert_eq!(x.integer_mul(n), F { a: n * x.a });
            }
        }
    }

    #[test]
    fn from_integer() {
        type F = PrimeField<23>;
        for x in -100..100 {
            assert_eq!(F::from_integer(x), F { a: x });
        }
        assert_eq!(F::from(0), F::ZERO);
        assert_eq!(F::from(1), F::ONE);
    }
}
