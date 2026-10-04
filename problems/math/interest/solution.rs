// value of e
use std::f64::consts::E;

// function to calculate simple interest
pub fn simple_interest(principal: f64, annual_rate: f64, years: f64) -> (f64, f64) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (simple_interest)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement simple_interest");
}

// function to calculate compound interest compounded over periods or continuously
pub fn compound_interest(principal: f64, annual_rate: f64, years: f64, period: Option<f64>) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (compound_interest)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement compound_interest");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        let x = 385.65_f64 * 0.03_f64 * 5.0_f64;
        let y = 385.65_f64 * (1.0 + (0.03_f64 * 5.0_f64));
        assert_eq!(simple_interest(385.65_f64, 0.03_f64, 5.0_f64), (x, y));
    }
    #[test]
    fn test_compounding() {
        let x = 385.65_f64 * E.powf(0.03_f64 * 5.0_f64);
        assert_eq!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, None), x);

        let y = 385.65_f64 * (1.0 + (0.03_f64 / 5.0_f64).powf(5.0_f64 * 5.0_f64));
        assert_eq!(
            compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(5.0_f64)),
            y
        );
        assert!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(-5.0_f64)).is_nan());
        assert!(compound_interest(385.65_f64, 0.03_f64, 5.0_f64, Some(0.0_f64)).is_nan());
    }
}
