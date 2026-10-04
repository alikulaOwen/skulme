// In computer science and statistics,
// the Jaro–Winkler distance is a string metric measuring an edit distance
// between two sequences.
// It is a variant proposed in 1990 by William E. Winkler
// of the Jaro distance metric (1989, Matthew A. Jaro).

pub fn jaro_winkler_distance(str1: &str, str2: &str) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (jaro_winkler_distance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement jaro_winkler_distance");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaro_winkler_distance() {
        let a = jaro_winkler_distance("hello", "world");
        assert_eq!(a, 0.4666666666666666);
        let a = jaro_winkler_distance("martha", "marhta");
        assert_eq!(a, 0.9611111111111111);
        let a = jaro_winkler_distance("martha", "marhat");
        assert_eq!(a, 0.9611111111111111);
        let a = jaro_winkler_distance("test", "test");
        assert_eq!(a, 1.0);
        let a = jaro_winkler_distance("test", "");
        assert_eq!(a, 0.0);
        let a = jaro_winkler_distance("hello world", "HeLLo W0rlD");
        assert_eq!(a, 0.6363636363636364);
    }
}
