// Author : cyrixninja
// Hexadecimal to Binary Converter : Converts Hexadecimal to Binary
// Wikipedia References  : 1. https://en.wikipedia.org/wiki/Hexadecimal
//                         2. https://en.wikipedia.org/wiki/Binary_number
// Other References for Testing : https://www.rapidtables.com/convert/number/hex-to-binary.html

pub fn hexadecimal_to_binary(hex_str: &str) -> Result<String, String> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (hexadecimal_to_binary)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement hexadecimal_to_binary");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let input = "";
        let expected = Ok("".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }

    #[test]
    fn test_hexadecimal() {
        let input = "1a2";
        let expected = Ok("000110100010".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }
    #[test]
    fn test_hexadecimal2() {
        let input = "1b3";
        let expected = Ok("000110110011".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }

    #[test]
    fn test_invalid_hexadecimal() {
        let input = "1g3";
        let expected = Err("Invalid".to_string());
        assert_eq!(hexadecimal_to_binary(input), expected);
    }
}
