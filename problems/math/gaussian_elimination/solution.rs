// Gaussian Elimination of Quadratic Matrices
// Takes an augmented matrix as input, returns vector of results
// Wikipedia reference: augmented matrix: https://en.wikipedia.org/wiki/Augmented_matrix
// Wikipedia reference: algorithm: https://en.wikipedia.org/wiki/Gaussian_elimination

pub fn gaussian_elimination(matrix: &mut [Vec<f32>]) -> Vec<f32> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gaussian_elimination)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gaussian_elimination");
}

fn echelon(matrix: &mut [Vec<f32>], i: usize, j: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (echelon)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement echelon");
}

fn eliminate(matrix: &mut [Vec<f32>], i: usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (eliminate)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement eliminate");
}


#[cfg(test)]
mod tests {
    use super::gaussian_elimination;

    #[test]
    fn test_gauss() {
        let mut matrix: Vec<Vec<f32>> = vec![
            vec![1.5, 2.0, 1.0, -1.0, -2.0, 1.0, 1.0],
            vec![3.0, 3.0, -1.0, 16.0, 18.0, 1.0, 1.0],
            vec![1.0, 1.0, 3.0, -2.0, -6.0, 1.0, 1.0],
            vec![1.0, 1.0, 99.0, 19.0, 2.0, 1.0, 1.0],
            vec![1.0, -2.0, 16.0, 1.0, 9.0, 10.0, 1.0],
            vec![1.0, 3.0, 1.0, -5.0, 1.0, 1.0, 95.0],
        ];
        let result = vec![
            -264.05893, 159.63196, -6.156921, 35.310387, -18.806696, 81.67839,
        ];
        assert_eq!(gaussian_elimination(&mut matrix), result);
    }
}
