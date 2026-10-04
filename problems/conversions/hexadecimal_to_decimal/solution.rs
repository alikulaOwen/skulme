pub fn hexadecimal_to_decimal(hexadecimal_str: &str) -> Result<u64, &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (hexadecimal_to_decimal)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement hexadecimal_to_decimal");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexadecimal_to_decimal_empty() {
        assert_eq!(hexadecimal_to_decimal(""), Err("Empty input"));
    }

    #[test]
    fn test_hexadecimal_to_decimal_invalid() {
        assert_eq!(
            hexadecimal_to_decimal("xyz"),
            Err("Input was not a hexadecimal number")
        );
        assert_eq!(
            hexadecimal_to_decimal("0xabc"),
            Err("Input was not a hexadecimal number")
        );
    }

    #[test]
    fn test_hexadecimal_to_decimal_valid1() {
        assert_eq!(hexadecimal_to_decimal("45"), Ok(69));
        assert_eq!(hexadecimal_to_decimal("2b3"), Ok(691));
        assert_eq!(hexadecimal_to_decimal("4d2"), Ok(1234));
        assert_eq!(hexadecimal_to_decimal("1267a"), Ok(75386));
    }

    #[test]
    fn test_hexadecimal_to_decimal_valid2() {
        assert_eq!(hexadecimal_to_decimal("1a"), Ok(26));
        assert_eq!(hexadecimal_to_decimal("ff"), Ok(255));
        assert_eq!(hexadecimal_to_decimal("a1b"), Ok(2587));
        assert_eq!(hexadecimal_to_decimal("7fffffff"), Ok(2147483647));
    }

    #[test]
    fn test_hexadecimal_to_decimal_valid3() {
        assert_eq!(hexadecimal_to_decimal("0"), Ok(0));
        assert_eq!(hexadecimal_to_decimal("7f"), Ok(127));
        assert_eq!(hexadecimal_to_decimal("80000000"), Ok(2147483648));
    }
}
