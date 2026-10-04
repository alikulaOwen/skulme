pub fn kernighan(n: u32) -> i32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (kernighan)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement kernighan");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_set_bits() {
        assert_eq!(kernighan(0b0000_0000_0000_0000_0000_0000_0000_1011), 3);
        assert_eq!(kernighan(0b0000_0000_0000_0000_0000_0000_1000_0000), 1);
        assert_eq!(kernighan(0b1111_1111_1111_1111_1111_1111_1111_1101), 31);
    }
}
