use std::f64::consts::E;

/// Calculates the **log<sub>base</sub>(x)**
///
/// Parameters:
///   <p>-> base: base of log
///   <p>-> x: value for which log shall be evaluated
///   <p>-> tol: tolerance; the precision of the approximation (submultiples of 10<sup>-1</sup>)
///
/// Advisable to use **std::f64::consts::*** for specific bases (like 'e')
pub fn log<T: Into<f64>, U: Into<f64>>(base: U, x: T, tol: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (log)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement log");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(log(E, E, 0.0), 1.0);
        assert_eq!(log(E, E.powi(100), 0.0), 100.0);
        assert_eq!(log(10, 10000.0, 0.0), 4.0);
        assert_eq!(log(234501.0, 1.0, 1.0), 0.0);
    }

    #[test]
    fn test_log_positive_base() {
        assert_eq!(log(10.0, 100.0, 0.00001), 2.0);
        assert_eq!(log(2.0, 8.0, 0.00001), 3.0);
    }

    #[test]
    fn test_log_zero_base() {
        assert!(log(0.0, 100.0, 0.00001).is_nan());
    }

    #[test]
    fn test_log_negative_base() {
        assert!(log(-1.0, 100.0, 0.00001).is_nan());
    }

    #[test]
    fn test_log_tolerance() {
        assert_eq!(log(10.0, 100.0, 1e-10), 2.0);
    }
}
