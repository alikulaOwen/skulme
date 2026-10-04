/// Returns the payback period in years
/// If investment is not paid back, returns None.

pub fn payback(cash_flow: &[f64]) -> Option<usize> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (payback)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement payback");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payback() {
        let cash_flows = vec![-1000.0, 300.0, 400.0, 500.0];
        assert_eq!(payback(&cash_flows), Some(3)); // paid back in year 3
    }

    #[test]
    fn test_no_payback() {
        let cash_flows = vec![-1000.0, 100.0, 100.0, 100.0];
        assert_eq!(payback(&cash_flows), None); // never paid back
    }
}
