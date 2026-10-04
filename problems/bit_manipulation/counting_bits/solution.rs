//! This module implements a function to count the number of set bits (1s)
//! in the binary representation of an unsigned integer.
//! It uses Brian Kernighan's algorithm, which efficiently clears the least significant
//! set bit in each iteration until all bits are cleared.
//! The algorithm runs in O(k), where k is the number of set bits.

/// Counts the number of set bits in an unsigned integer.
///
/// # Arguments
///
/// * `n` - An unsigned 32-bit integer whose set bits will be counted.
///
/// # Returns
///
/// * `usize` - The number of set bits (1s) in the binary representation of the input number.
pub fn count_set_bits(mut n: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (count_set_bits)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement count_set_bits");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_count_set_bits {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $test_case;
                    assert_eq!(count_set_bits(input), expected);
                }
            )*
        };
    }
    test_count_set_bits! {
        test_count_set_bits_zero: (0, 0),
        test_count_set_bits_one: (1, 1),
        test_count_set_bits_power_of_two: (16, 1),
        test_count_set_bits_all_set_bits: (usize::MAX, std::mem::size_of::<usize>() * 8),
        test_count_set_bits_alternating_bits: (0b10101010, 4),
        test_count_set_bits_mixed_bits: (0b11011011, 6),
    }
}
