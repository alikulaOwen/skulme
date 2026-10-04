/**
 * Sprague Grundy Theorem for combinatorial games like Nim
 *
 * The Sprague Grundy Theorem is a fundamental concept in combinatorial game theory, commonly used to analyze
 * games like Nim. It calculates the Grundy number (also known as the nimber) for a position in a game.
 * The Grundy number represents the game's position, and it helps determine the winning strategy.
 *
 * The Grundy number of a terminal state is 0; otherwise, it is recursively defined as the minimum
 * excludant (mex) of the Grundy values of possible next states.
 *
 * For more details on Sprague Grundy Theorem, you can visit:(https://en.wikipedia.org/wiki/Sprague%E2%80%93Grundy_theorem)
 *
 * Author : [Gyandeep](https://github.com/Gyan172004)
 */

pub fn calculate_grundy_number(
    position: i64,
    grundy_numbers: &mut [i64],
    possible_moves: &[i64],
) -> i64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (calculate_grundy_number)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement calculate_grundy_number");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_grundy_number_test() {
        let mut grundy_numbers: Vec<i64> = vec![-1; 7];
        let possible_moves: Vec<i64> = vec![1, 4];
        calculate_grundy_number(6, &mut grundy_numbers, &possible_moves);
        assert_eq!(grundy_numbers, [0, 1, 0, 1, 2, 0, 1]);
    }
}
