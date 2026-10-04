pub fn manacher(s: String) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (manacher)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement manacher");
}


#[cfg(test)]
mod tests {
    use super::manacher;

    #[test]
    fn get_longest_palindrome_by_manacher() {
        assert_eq!(manacher("babad".to_string()), "aba".to_string());
        assert_eq!(manacher("cbbd".to_string()), "bb".to_string());
        assert_eq!(manacher("a".to_string()), "a".to_string());

        let ac_ans = manacher("ac".to_string());
        assert!(ac_ans == *"a" || ac_ans == *"c");
    }
}
