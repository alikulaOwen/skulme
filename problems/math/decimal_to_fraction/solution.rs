pub fn decimal_to_fraction(decimal: f64) -> (i64, i64) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decimal_to_fraction)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decimal_to_fraction");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_fraction_1() {
        assert_eq!(decimal_to_fraction(2.0), (2, 1));
    }

    #[test]
    fn test_decimal_to_fraction_2() {
        assert_eq!(decimal_to_fraction(89.45), (1789, 20));
    }

    #[test]
    fn test_decimal_to_fraction_3() {
        assert_eq!(decimal_to_fraction(67.), (67, 1));
    }

    #[test]
    fn test_decimal_to_fraction_4() {
        assert_eq!(decimal_to_fraction(45.2), (226, 5));
    }

    #[test]
    fn test_decimal_to_fraction_5() {
        assert_eq!(decimal_to_fraction(1.5), (3, 2));
    }

    #[test]
    fn test_decimal_to_fraction_6() {
        assert_eq!(decimal_to_fraction(6.25), (25, 4));
    }
}
