// in theory rot-13 only affects the lowercase characters in a cipher
pub fn theoretical_rot13(text: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (theoretical_rot13)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement theoretical_rot13");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_letter() {
        assert_eq!("n", theoretical_rot13("a"));
    }
    #[test]
    fn test_bunch_of_letters() {
        assert_eq!("nop op", theoretical_rot13("abc bc"));
    }

    #[test]
    fn test_non_ascii() {
        assert_eq!("😀ab", theoretical_rot13("😀no"));
    }

    #[test]
    fn test_twice() {
        assert_eq!("abcd", theoretical_rot13(&theoretical_rot13("abcd")));
    }
}
