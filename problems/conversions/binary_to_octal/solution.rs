// Author: NithinU2802
// Binary to Octal Converter: Converts Binary to Octal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Binary_number
// 2. https://en.wikipedia.org/wiki/Octal

pub fn binary_to_octal(binary_str: &str) -> Result<String, &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (binary_to_octal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement binary_to_octal");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_octal() {
        assert_eq!(binary_to_octal("1010"), Ok("12".to_string()));
        assert_eq!(binary_to_octal("1111"), Ok("17".to_string()));
        assert_eq!(binary_to_octal("11111111"), Ok("377".to_string()));
        assert_eq!(binary_to_octal("1100100"), Ok("144".to_string()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(binary_to_octal(""), Err("Empty string"));
        assert_eq!(binary_to_octal("12"), Err("Invalid binary string"));
        assert_eq!(binary_to_octal("abc"), Err("Invalid binary string"));
    }
}
