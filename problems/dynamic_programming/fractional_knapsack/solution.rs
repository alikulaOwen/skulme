pub fn fractional_knapsack(mut capacity: f64, weights: Vec<f64>, values: Vec<f64>) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fractional_knapsack)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fractional_knapsack");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let capacity = 50.0;
        let values = vec![60.0, 100.0, 120.0];
        let weights = vec![10.0, 20.0, 30.0];
        assert_eq!(fractional_knapsack(capacity, weights, values), 240.0);
    }

    #[test]
    fn test2() {
        let capacity = 60.0;
        let values = vec![280.0, 100.0, 120.0, 120.0];
        let weights = vec![40.0, 10.0, 20.0, 24.0];
        assert_eq!(fractional_knapsack(capacity, weights, values), 440.0);
    }

    #[test]
    fn test3() {
        let capacity = 50.0;
        let values = vec![60.0, 100.0, 120.0];
        let weights = vec![20.0, 50.0, 30.0];
        assert_eq!(fractional_knapsack(capacity, weights, values), 180.0);
    }

    #[test]
    fn test4() {
        let capacity = 60.0;
        let values = vec![30.0, 40.0, 45.0, 77.0, 90.0];
        let weights = vec![5.0, 10.0, 15.0, 22.0, 25.0];
        assert_eq!(fractional_knapsack(capacity, weights, values), 230.0);
    }

    #[test]
    fn test5() {
        let capacity = 10.0;
        let values = vec![500.0];
        let weights = vec![30.0];
        assert_eq!(
            format!("{:.2}", fractional_knapsack(capacity, weights, values)),
            String::from("166.67")
        );
    }

    #[test]
    fn test6() {
        let capacity = 36.0;
        let values = vec![25.0, 25.0, 25.0, 6.0, 2.0];
        let weights = vec![10.0, 10.0, 10.0, 4.0, 2.0];
        assert_eq!(fractional_knapsack(capacity, weights, values), 83.0);
    }

    #[test]
    #[should_panic]
    fn test_nan() {
        let capacity = 36.0;
        // 2nd element is NaN
        let values = vec![25.0, f64::NAN, 25.0, 6.0, 2.0];
        let weights = vec![10.0, 10.0, 10.0, 4.0, 2.0];
        assert_eq!(fractional_knapsack(capacity, weights, values), 83.0);
    }
}
