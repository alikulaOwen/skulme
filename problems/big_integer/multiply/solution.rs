/// Performs long multiplication on string representations of non-negative numbers.
pub fn multiply(num1: &str, num2: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (multiply)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement multiply");
}

pub fn is_valid_nonnegative(num: &str) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (is_valid_nonnegative)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement is_valid_nonnegative");
}


#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! test_multiply {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, t, expected) = $inputs;
                assert_eq!(multiply(s, t), expected);
                assert_eq!(multiply(t, s), expected);
            }
        )*
        }
    }

    test_multiply! {
        multiply0: ("2", "3", "6"),
        multiply1: ("123", "456", "56088"),
        multiply_zero: ("0", "222", "0"),
        other_1: ("99", "99", "9801"),
        other_2: ("999", "99", "98901"),
        other_3: ("9999", "99", "989901"),
        other_4: ("192939", "9499596", "1832842552644"),
    }

    macro_rules! test_multiply_with_wrong_input {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (s, t) = $inputs;
                multiply(s, t);
            }
        )*
        }
    }
    test_multiply_with_wrong_input! {
        empty_input: ("", "121"),
        leading_zero: ("01", "3"),
        wrong_characters: ("2", "12d4"),
        wrong_input_and_zero_1: ("0", "x"),
        wrong_input_and_zero_2: ("y", "0"),
    }
}
