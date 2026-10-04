/// Fibonacci via Dynamic Programming
use std::collections::HashMap;

/// fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn fibonacci(n: u32) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement fibonacci");
}

/// fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn recursive_fibonacci(n: u32) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (recursive_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement recursive_fibonacci");
}

fn _recursive_fibonacci(n: u32, previous: u128, current: u128) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_recursive_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _recursive_fibonacci");
}

/// classical_fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn classical_fibonacci(n: u32) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (classical_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement classical_fibonacci");
}

/// logarithmic_fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn logarithmic_fibonacci(n: u32) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (logarithmic_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement logarithmic_fibonacci");
}

fn _logarithmic_fibonacci(n: u32) -> (u128, u128) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_logarithmic_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _logarithmic_fibonacci");
}

/// Memoized fibonacci.
pub fn memoized_fibonacci(n: u32) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (memoized_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement memoized_fibonacci");
}

fn _memoized_fibonacci(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_memoized_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _memoized_fibonacci");
}

/// matrix_fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Matrix formula:
/// [F(n + 2)]  =  [1, 1] * [F(n + 1)]
/// [F(n + 1)]     [1, 0]   [F(n)    ]
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn matrix_fibonacci(n: u32) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (matrix_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement matrix_fibonacci");
}

fn matrix_power(base: &Vec<Vec<u128>>, power: u32) -> Vec<Vec<u128>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (matrix_power)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement matrix_power");
}

// Copied from matrix_ops since u128 is required instead of i32
#[allow(clippy::needless_range_loop)]
fn matrix_multiply(multiplier: &[Vec<u128>], multiplicand: &[Vec<u128>]) -> Vec<Vec<u128>> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (matrix_multiply)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement matrix_multiply");
}

/// Binary lifting fibonacci
///
/// Following properties of F(n) could be deduced from the matrix formula above:
///
/// F(2n)   = F(n) * (2F(n+1) - F(n))
/// F(2n+1) = F(n+1)^2 + F(n)^2
///
/// Therefore F(n) and F(n+1) can be derived from F(n>>1) and F(n>>1 + 1), which
/// has a smaller constant in both time and space compared to matrix fibonacci.
pub fn binary_lifting_fibonacci(n: u32) -> u128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (binary_lifting_fibonacci)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement binary_lifting_fibonacci");
}

/// nth_fibonacci_number_modulo_m(n, m) returns the nth fibonacci number modulo the specified m
/// i.e. F(n) % m
pub fn nth_fibonacci_number_modulo_m(n: i64, m: i64) -> i128 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (nth_fibonacci_number_modulo_m)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement nth_fibonacci_number_modulo_m");
}

/// get_pisano_sequence_and_period(m) returns the Pisano Sequence and period for the specified integer m.
/// The pisano period is the period with which the sequence of Fibonacci numbers taken modulo m repeats.
/// The pisano sequence is the numbers in pisano period.
fn get_pisano_sequence_and_period(m: i64) -> (i128, Vec<i128>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_pisano_sequence_and_period)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_pisano_sequence_and_period");
}

/// last_digit_of_the_sum_of_nth_fibonacci_number(n) returns the last digit of the sum of n fibonacci numbers.
/// The function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n > 2
///
/// The sum of the Fibonacci numbers are:
/// F(0) + F(1) + F(2) + ... + F(n)
pub fn last_digit_of_the_sum_of_nth_fibonacci_number(n: i64) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (last_digit_of_the_sum_of_nth_fibonacci_number)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement last_digit_of_the_sum_of_nth_fibonacci_number");
}


#[cfg(test)]
mod tests {
    use super::binary_lifting_fibonacci;
    use super::classical_fibonacci;
    use super::fibonacci;
    use super::last_digit_of_the_sum_of_nth_fibonacci_number;
    use super::logarithmic_fibonacci;
    use super::matrix_fibonacci;
    use super::memoized_fibonacci;
    use super::nth_fibonacci_number_modulo_m;
    use super::recursive_fibonacci;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 1);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(4), 5);
        assert_eq!(fibonacci(5), 8);
        assert_eq!(fibonacci(10), 89);
        assert_eq!(fibonacci(20), 10946);
        assert_eq!(fibonacci(100), 573147844013817084101);
        assert_eq!(fibonacci(184), 205697230343233228174223751303346572685);
    }

    #[test]
    fn test_recursive_fibonacci() {
        assert_eq!(recursive_fibonacci(0), 1);
        assert_eq!(recursive_fibonacci(1), 1);
        assert_eq!(recursive_fibonacci(2), 2);
        assert_eq!(recursive_fibonacci(3), 3);
        assert_eq!(recursive_fibonacci(4), 5);
        assert_eq!(recursive_fibonacci(5), 8);
        assert_eq!(recursive_fibonacci(10), 89);
        assert_eq!(recursive_fibonacci(20), 10946);
        assert_eq!(recursive_fibonacci(100), 573147844013817084101);
        assert_eq!(
            recursive_fibonacci(184),
            205697230343233228174223751303346572685
        );
    }

    #[test]
    fn test_classical_fibonacci() {
        assert_eq!(classical_fibonacci(0), 0);
        assert_eq!(classical_fibonacci(1), 1);
        assert_eq!(classical_fibonacci(2), 1);
        assert_eq!(classical_fibonacci(3), 2);
        assert_eq!(classical_fibonacci(4), 3);
        assert_eq!(classical_fibonacci(5), 5);
        assert_eq!(classical_fibonacci(10), 55);
        assert_eq!(classical_fibonacci(20), 6765);
        assert_eq!(classical_fibonacci(21), 10946);
        assert_eq!(classical_fibonacci(100), 354224848179261915075);
        assert_eq!(
            classical_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_logarithmic_fibonacci() {
        assert_eq!(logarithmic_fibonacci(0), 0);
        assert_eq!(logarithmic_fibonacci(1), 1);
        assert_eq!(logarithmic_fibonacci(2), 1);
        assert_eq!(logarithmic_fibonacci(3), 2);
        assert_eq!(logarithmic_fibonacci(4), 3);
        assert_eq!(logarithmic_fibonacci(5), 5);
        assert_eq!(logarithmic_fibonacci(10), 55);
        assert_eq!(logarithmic_fibonacci(20), 6765);
        assert_eq!(logarithmic_fibonacci(21), 10946);
        assert_eq!(logarithmic_fibonacci(100), 354224848179261915075);
        assert_eq!(
            logarithmic_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    /// Check that the iterative and recursive fibonacci
    /// produce the same value. Both are combinatorial ( F(0) = F(1) = 1 )
    fn test_iterative_and_recursive_equivalence() {
        assert_eq!(fibonacci(0), recursive_fibonacci(0));
        assert_eq!(fibonacci(1), recursive_fibonacci(1));
        assert_eq!(fibonacci(2), recursive_fibonacci(2));
        assert_eq!(fibonacci(3), recursive_fibonacci(3));
        assert_eq!(fibonacci(4), recursive_fibonacci(4));
        assert_eq!(fibonacci(5), recursive_fibonacci(5));
        assert_eq!(fibonacci(10), recursive_fibonacci(10));
        assert_eq!(fibonacci(20), recursive_fibonacci(20));
        assert_eq!(fibonacci(100), recursive_fibonacci(100));
        assert_eq!(fibonacci(184), recursive_fibonacci(184));
    }

    #[test]
    /// Check that classical and combinatorial fibonacci produce the
    /// same value when 'n' differs by 1.
    /// classical fibonacci: ( F(0) = 0, F(1) = 1 )
    /// combinatorial fibonacci: ( F(0) = F(1) = 1 )
    fn test_classical_and_combinatorial_are_off_by_one() {
        assert_eq!(classical_fibonacci(1), fibonacci(0));
        assert_eq!(classical_fibonacci(2), fibonacci(1));
        assert_eq!(classical_fibonacci(3), fibonacci(2));
        assert_eq!(classical_fibonacci(4), fibonacci(3));
        assert_eq!(classical_fibonacci(5), fibonacci(4));
        assert_eq!(classical_fibonacci(6), fibonacci(5));
        assert_eq!(classical_fibonacci(11), fibonacci(10));
        assert_eq!(classical_fibonacci(20), fibonacci(19));
        assert_eq!(classical_fibonacci(21), fibonacci(20));
        assert_eq!(classical_fibonacci(101), fibonacci(100));
        assert_eq!(classical_fibonacci(185), fibonacci(184));
    }

    #[test]
    fn test_memoized_fibonacci() {
        assert_eq!(memoized_fibonacci(0), 0);
        assert_eq!(memoized_fibonacci(1), 1);
        assert_eq!(memoized_fibonacci(2), 1);
        assert_eq!(memoized_fibonacci(3), 2);
        assert_eq!(memoized_fibonacci(4), 3);
        assert_eq!(memoized_fibonacci(5), 5);
        assert_eq!(memoized_fibonacci(10), 55);
        assert_eq!(memoized_fibonacci(20), 6765);
        assert_eq!(memoized_fibonacci(21), 10946);
        assert_eq!(memoized_fibonacci(100), 354224848179261915075);
        assert_eq!(
            memoized_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_matrix_fibonacci() {
        assert_eq!(matrix_fibonacci(0), 0);
        assert_eq!(matrix_fibonacci(1), 1);
        assert_eq!(matrix_fibonacci(2), 1);
        assert_eq!(matrix_fibonacci(3), 2);
        assert_eq!(matrix_fibonacci(4), 3);
        assert_eq!(matrix_fibonacci(5), 5);
        assert_eq!(matrix_fibonacci(10), 55);
        assert_eq!(matrix_fibonacci(20), 6765);
        assert_eq!(matrix_fibonacci(21), 10946);
        assert_eq!(matrix_fibonacci(100), 354224848179261915075);
        assert_eq!(
            matrix_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_binary_lifting_fibonacci() {
        assert_eq!(binary_lifting_fibonacci(0), 0);
        assert_eq!(binary_lifting_fibonacci(1), 1);
        assert_eq!(binary_lifting_fibonacci(2), 1);
        assert_eq!(binary_lifting_fibonacci(3), 2);
        assert_eq!(binary_lifting_fibonacci(4), 3);
        assert_eq!(binary_lifting_fibonacci(5), 5);
        assert_eq!(binary_lifting_fibonacci(10), 55);
        assert_eq!(binary_lifting_fibonacci(20), 6765);
        assert_eq!(binary_lifting_fibonacci(21), 10946);
        assert_eq!(binary_lifting_fibonacci(100), 354224848179261915075);
        assert_eq!(
            binary_lifting_fibonacci(184),
            127127879743834334146972278486287885163
        );
    }

    #[test]
    fn test_nth_fibonacci_number_modulo_m() {
        assert_eq!(nth_fibonacci_number_modulo_m(5, 10), 5);
        assert_eq!(nth_fibonacci_number_modulo_m(10, 7), 6);
        assert_eq!(nth_fibonacci_number_modulo_m(20, 100), 65);
        assert_eq!(nth_fibonacci_number_modulo_m(1, 5), 1);
        assert_eq!(nth_fibonacci_number_modulo_m(0, 15), 0);
        assert_eq!(nth_fibonacci_number_modulo_m(50, 1000), 25);
        assert_eq!(nth_fibonacci_number_modulo_m(100, 37), 7);
        assert_eq!(nth_fibonacci_number_modulo_m(15, 2), 0);
        assert_eq!(nth_fibonacci_number_modulo_m(8, 1_000_000), 21);
        assert_eq!(nth_fibonacci_number_modulo_m(1000, 997), 996);
        assert_eq!(nth_fibonacci_number_modulo_m(200, 123), 0);
    }

    #[test]
    fn test_last_digit_of_the_sum_of_nth_fibonacci_number() {
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(0), 0);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(1), 1);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(2), 2);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(3), 4);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(4), 7);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(5), 2);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(25), 7);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(50), 8);
        assert_eq!(last_digit_of_the_sum_of_nth_fibonacci_number(100), 5);
    }
}
