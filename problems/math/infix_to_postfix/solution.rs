#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixToPostfixError {
    UnknownCharacter(char),
    UnmatchedParent,
}

/// Function to convert [infix expression](https://en.wikipedia.org/wiki/Infix_notation) to [postfix expression](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
pub fn infix_to_postfix(infix: &str) -> Result<String, InfixToPostfixError> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (infix_to_postfix)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement infix_to_postfix");
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_infix_to_postfix {
        ($($name:ident: $inputs:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (infix, expected) = $inputs;
                    assert_eq!(infix_to_postfix(infix), expected)
                }
            )*
        }
    }

    test_infix_to_postfix! {
        single_symbol: ("x", Ok(String::from("x"))),
        simple_sum: ("x+y", Ok(String::from("xy+"))),
        multiply_sum_left: ("x*(y+z)", Ok(String::from("xyz+*"))),
        multiply_sum_right: ("(x+y)*z", Ok(String::from("xy+z*"))),
        multiply_two_sums: ("(a+b)*(c+d)", Ok(String::from("ab+cd+*"))),
        product_and_power: ("a*b^c", Ok(String::from("abc^*"))),
        power_and_product: ("a^b*c", Ok(String::from("ab^c*"))),
        product_of_powers: ("(a*b)^c", Ok(String::from("ab*c^"))),
        product_in_exponent: ("a^(b*c)", Ok(String::from("abc*^"))),
        regular_0: ("a-b+c-d*e", Ok(String::from("ab-c+de*-"))),
        regular_1: ("a*(b+c)+d/(e+f)", Ok(String::from("abc+*def+/+"))),
        regular_2: ("(a-b+c)*(d+e*f)", Ok(String::from("ab-c+def*+*"))),
        unknown_character: ("(a-b)*#", Err(InfixToPostfixError::UnknownCharacter('#'))),
        unmatched_paren: ("((a-b)", Err(InfixToPostfixError::UnmatchedParent)),
    }
}
