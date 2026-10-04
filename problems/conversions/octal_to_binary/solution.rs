// Author : cyrixninja
// Octal to Binary Converter : Converts Octal to Binary
// Wikipedia References  : 1. https://en.wikipedia.org/wiki/Octal
//                         2. https://en.wikipedia.org/wiki/Binary_number

pub fn octal_to_binary(octal_str: &str) -> Result<String, &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (octal_to_binary)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement octal_to_binary");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = Err("Empty");
        assert_eq!(octal_to_binary(input), expected);
    }

    #[test]
    fn test_invalid_octal() {
        let input = "89";
        let expected = Err("Non-octal Value");
        assert_eq!(octal_to_binary(input), expected);
    }

    #[test]
    fn test_valid_octal() {
        let input = "123";
        let expected = Ok("001010011".to_string());
        assert_eq!(octal_to_binary(input), expected);
    }
}
