// Author: NithinU2802
// Decimal to Octal Converter: Converts Decimal to Octal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Decimal
// 2. https://en.wikipedia.org/wiki/Octal

pub fn decimal_to_octal(decimal_num: u64) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decimal_to_octal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decimal_to_octal");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_octal() {
        assert_eq!(decimal_to_octal(8), "10");
        assert_eq!(decimal_to_octal(15), "17");
        assert_eq!(decimal_to_octal(255), "377");
        assert_eq!(decimal_to_octal(100), "144");
        assert_eq!(decimal_to_octal(0), "0");
    }
}
