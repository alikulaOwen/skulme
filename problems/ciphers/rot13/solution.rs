pub fn rot13(text: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rot13)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rot13");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_letter() {
        assert_eq!("N", rot13("A"));
    }

    #[test]
    fn test_bunch_of_letters() {
        assert_eq!("NOP", rot13("ABC"));
    }

    #[test]
    fn test_non_ascii() {
        assert_eq!("😀NO", rot13("😀AB"));
    }

    #[test]
    fn test_twice() {
        assert_eq!("ABCD", rot13(&rot13("ABCD")));
    }
}
