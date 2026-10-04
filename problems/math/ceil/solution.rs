// In mathematics and computer science, the ceiling function maps x to the least integer greater than or equal to x
// Source: https://en.wikipedia.org/wiki/Floor_and_ceiling_functions

pub fn ceil(x: f64) -> f64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (ceil)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement ceil");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimal() {
        let num = 1.10;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_decimal_with_small_number() {
        let num = 3.01;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn positive_integer() {
        let num = 1.00;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal() {
        let num = -1.10;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_decimal_with_small_number() {
        let num = -1.01;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn negative_integer() {
        let num = -1.00;
        assert_eq!(ceil(num), num.ceil());
    }

    #[test]
    fn zero() {
        let num = 0.00;
        assert_eq!(ceil(num), num.ceil());
    }
}
