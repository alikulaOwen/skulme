pub fn is_armstrong_number(number: u32) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_armstrong_number)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_armstrong_number");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit_armstrong_number() {
        assert!(is_armstrong_number(1))
    }
    #[test]
    fn two_digit_numbers_are_not_armstrong_numbers() {
        assert!(!is_armstrong_number(15))
    }
    #[test]
    fn three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }
    #[test]
    fn three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(105))
    }
    #[test]
    fn big_armstrong_number() {
        assert!(is_armstrong_number(912985153))
    }
}
