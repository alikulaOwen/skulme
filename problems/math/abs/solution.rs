/// This function returns the absolute value of a number.\
/// The absolute value of a number is the non-negative value of the number, regardless of its sign.\
///
/// Wikipedia: <https://en.wikipedia.org/wiki/Absolute_value>
pub fn abs<T>(num: T) -> T
where
    T: std::ops::Neg<Output = T> + PartialOrd + Copy + num_traits::Zero,
{
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (abs)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement abs");
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_negative_number_i32() {
        assert_eq!(69, abs(-69));
    }

    #[test]
    fn test_negative_number_f64() {
        assert_eq!(69.69, abs(-69.69));
    }

    #[test]
    fn zero() {
        assert_eq!(0.0, abs(0.0));
    }

    #[test]
    fn positive_number() {
        assert_eq!(69.69, abs(69.69));
    }
}
