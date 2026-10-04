fn floor(value: f64, scale: u8) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (floor)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement floor");
}

fn double_to_int(amount: f64) -> i128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (double_to_int)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement double_to_int");
}

pub fn trial_division(mut num: i128) -> Vec<i128> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (trial_division)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement trial_division");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(trial_division(0), vec![]);
        assert_eq!(trial_division(1), vec![]);
        assert_eq!(trial_division(9), vec!(3, 3));
        assert_eq!(trial_division(-9), vec!(3, 3));
        assert_eq!(trial_division(10), vec!(2, 5));
        assert_eq!(trial_division(11), vec!(11));
        assert_eq!(trial_division(33), vec!(3, 11));
        assert_eq!(trial_division(2003), vec!(2003));
        assert_eq!(trial_division(100001), vec!(11, 9091));
    }
}
