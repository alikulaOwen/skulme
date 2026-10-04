use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::sync::RwLock;

/// Returns the number of ways you can select r items given n options
fn n_choose_r(n: u32, r: u32) -> BigUint {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (n_choose_r)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement n_choose_r");
}

/// A memoization table for storing previous results
struct MemTable {
    buffer: Vec<BigUint>,
}

impl MemTable {
    const fn new() -> Self {
        MemTable { buffer: Vec::new() }
    }

    fn get(&self, n: usize) -> Option<BigUint> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get");
}

    fn set(&mut self, n: usize, b: BigUint) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (set)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement set");
}

    #[inline]
    fn capacity(&self) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (capacity)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement capacity");
}

    #[inline]
    fn resize(&mut self, new_size: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (resize)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement resize");
}
}

// Implemented with RwLock so it is accessible across threads
static LOOKUP_TABLE_LOCK: RwLock<MemTable> = RwLock::new(MemTable::new());

pub fn bell_number(n: u32) -> BigUint {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (bell_number)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement bell_number");
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_choose_zero() {
        for i in 1..100 {
            assert_eq!(n_choose_r(i, 0), One::one());
        }
    }

    #[test]
    fn test_combination() {
        let five_choose_1 = BigUint::from(5u32);
        assert_eq!(n_choose_r(5, 1), five_choose_1);
        assert_eq!(n_choose_r(5, 4), five_choose_1);

        let ten_choose_3 = BigUint::from(120u32);
        assert_eq!(n_choose_r(10, 3), ten_choose_3);
        assert_eq!(n_choose_r(10, 7), ten_choose_3);

        let fourty_two_choose_thirty = BigUint::from_str("11058116888").unwrap();
        assert_eq!(n_choose_r(42, 30), fourty_two_choose_thirty);
        assert_eq!(n_choose_r(42, 12), fourty_two_choose_thirty);
    }

    #[test]
    fn test_bell_numbers() {
        let bell_one = BigUint::from(1u32);
        assert_eq!(bell_number(1), bell_one);

        let bell_three = BigUint::from(5u32);
        assert_eq!(bell_number(3), bell_three);

        let bell_eight = BigUint::from(4140u32);
        assert_eq!(bell_number(8), bell_eight);

        let bell_six = BigUint::from(203u32);
        assert_eq!(bell_number(6), bell_six);

        let bell_twenty_six = BigUint::from_str("49631246523618756274").unwrap();
        assert_eq!(bell_number(26), bell_twenty_six);
    }
}
