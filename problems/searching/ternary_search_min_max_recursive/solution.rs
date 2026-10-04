/// Recursive ternary search algorithm for finding maximum of unimodal function
pub fn ternary_search_max_rec(
    f: fn(f32) -> f32,
    start: f32,
    end: f32,
    absolute_precision: f32,
) -> f32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (ternary_search_max_rec)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement ternary_search_max_rec");
}

/// Recursive ternary search algorithm for finding minimum of unimodal function
pub fn ternary_search_min_rec(
    f: fn(f32) -> f32,
    start: f32,
    end: f32,
    absolute_precision: f32,
) -> f32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (ternary_search_min_rec)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement ternary_search_min_rec");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_value() {
        let expected = 4.0;
        let f = |x: f32| -x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.0000001;

        let result = ternary_search_max_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value() {
        let expected = 2.0;
        let f = |x: f32| x * x - 2.0 * x + 3.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.0000001;

        let result = ternary_search_min_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_max_value_2() {
        let expected = 7.25;
        let f = |x: f32| -x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_max_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }

    #[test]
    fn finds_min_value_2() {
        let expected = 2.75;
        let f = |x: f32| x.powi(2) + 3.0 * x + 5.0;

        let start: f32 = -10000000000.0;
        let end: f32 = 10000000000.0;
        let absolute_precision = 0.000001;

        let result = ternary_search_min_rec(f, start, end, absolute_precision);

        assert_eq!(result, expected);
    }
}
