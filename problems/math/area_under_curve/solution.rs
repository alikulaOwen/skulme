pub fn area_under_curve(start: f64, end: f64, func: fn(f64) -> f64, step_count: usize) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (area_under_curve)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement area_under_curve");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_linear_func() {
        assert_eq!(area_under_curve(1f64, 2f64, |x| x, 10), 1.5000000000000002);
    }

    #[test]
    fn test_quadratic_func() {
        assert_eq!(
            area_under_curve(1f64, 2f64, |x| x * x, 1000),
            2.333333500000005
        );
    }

    #[test]
    fn test_zero_length() {
        assert_eq!(area_under_curve(0f64, 0f64, |x| x * x, 1000), 0.0);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(
            area_under_curve(1f64, 2f64, |x| x, 10),
            area_under_curve(2f64, 1f64, |x| x, 10)
        );
    }
}
