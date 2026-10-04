// Author: NithinU2802
// Hexadecimal to Octal Converter: Converts Hexadecimal to Octal
// Wikipedia References:
// 1. https://en.wikipedia.org/wiki/Hexadecimal
// 2. https://en.wikipedia.org/wiki/Octal

pub fn hexadecimal_to_octal(hex_str: &str) -> Result<String, &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (hexadecimal_to_octal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement hexadecimal_to_octal");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexadecimal_to_octal() {
        assert_eq!(hexadecimal_to_octal("A"), Ok("12".to_string()));
        assert_eq!(hexadecimal_to_octal("FF"), Ok("377".to_string()));
        assert_eq!(hexadecimal_to_octal("64"), Ok("144".to_string()));
        assert_eq!(hexadecimal_to_octal("0"), Ok("0".to_string()));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(hexadecimal_to_octal(""), Err("Empty string"));
        assert_eq!(
            hexadecimal_to_octal("GG"),
            Err("Invalid hexadecimal string")
        );
        assert_eq!(
            hexadecimal_to_octal("XYZ"),
            Err("Invalid hexadecimal string")
        );
    }
}
