pub fn decimal_to_binary(base_num: u64) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decimal_to_binary)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decimal_to_binary");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converting_decimal_to_binary() {
        assert_eq!(decimal_to_binary(542), "1000011110");
        assert_eq!(decimal_to_binary(92), "1011100");
    }
}
