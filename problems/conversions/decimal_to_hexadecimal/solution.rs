pub fn decimal_to_hexadecimal(base_num: u64) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decimal_to_hexadecimal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decimal_to_hexadecimal");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert_eq!(decimal_to_hexadecimal(0), "0");
    }

    #[test]
    fn test_single_digit_decimal() {
        assert_eq!(decimal_to_hexadecimal(9), "9");
    }

    #[test]
    fn test_single_digit_hexadecimal() {
        assert_eq!(decimal_to_hexadecimal(12), "C");
    }

    #[test]
    fn test_multiple_digit_hexadecimal() {
        assert_eq!(decimal_to_hexadecimal(255), "FF");
    }

    #[test]
    fn test_big() {
        assert_eq!(decimal_to_hexadecimal(u64::MAX), "FFFFFFFFFFFFFFFF");
    }

    #[test]
    fn test_random() {
        assert_eq!(decimal_to_hexadecimal(123456), "1E240");
    }
}
