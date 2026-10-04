/// squre_root returns the square root
/// of a f64 number using Newton's method
pub fn square_root(num: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (square_root)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement square_root");
}

// fast_inv_sqrt returns an approximation of the inverse square root
// This algorithm was first used in Quake and has been reimplemented in a few other languages
// This crate implements it more thoroughly: https://docs.rs/quake-inverse-sqrt/latest/quake_inverse_sqrt/
pub fn fast_inv_sqrt(num: f32) -> f32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fast_inv_sqrt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fast_inv_sqrt");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_inv_sqrt() {
        // Negatives don't have square roots:
        assert!(fast_inv_sqrt(-1.0f32).is_nan());

        // Test a few cases, expect less than 1% error:
        let test_pairs = [(4.0, 0.5), (16.0, 0.25), (25.0, 0.2)];
        for pair in test_pairs {
            assert!((fast_inv_sqrt(pair.0) - pair.1).abs() <= (0.01 * pair.0));
        }
    }

    #[test]
    fn test_sqare_root() {
        assert!((square_root(4.0_f64) - 2.0_f64).abs() <= 1e-10_f64);
        assert!(square_root(-4.0_f64).is_nan());
    }
}
