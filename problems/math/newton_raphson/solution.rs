pub fn find_root(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64, iterations: i32) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (find_root)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement find_root");
}

pub fn iteration(f: fn(f64) -> f64, fd: fn(f64) -> f64, guess: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iteration)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iteration");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn math_fn(x: f64) -> f64 {
        x.cos() - (x * x * x)
    }
    fn math_fnd(x: f64) -> f64 {
        -x.sin() - 3.0 * (x * x)
    }
    #[test]
    fn basic() {
        assert_eq!(find_root(math_fn, math_fnd, 0.5, 6), 0.8654740331016144);
    }
}
