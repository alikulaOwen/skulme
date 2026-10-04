// Author: NithinU2802
// Octal to Hexadecimal Converter: Converts Octal to Hexadecimal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Octal
// 2. https://en.wikipedia.org/wiki/Hexadecimal

pub fn octal_to_hexadecimal(octal_str: &str) -> Result<String, &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (octal_to_hexadecimal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement octal_to_hexadecimal");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octal_to_hexadecimal() {
        assert_eq!(octal_to_hexadecimal("12"), Ok("A".to_string()));
        assert_eq!(octal_to_hexadecimal("377"), Ok("FF".to_string()));
        assert_eq!(octal_to_hexadecimal("144"), Ok("64".to_string()));
        assert_eq!(octal_to_hexadecimal("0"), Ok("0".to_string()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(octal_to_hexadecimal(""), Err("Empty string"));
        assert_eq!(octal_to_hexadecimal("8"), Err("Invalid octal string"));
        assert_eq!(octal_to_hexadecimal("9"), Err("Invalid octal string"));
        assert_eq!(octal_to_hexadecimal("ABC"), Err("Invalid octal string"));
    }
}
