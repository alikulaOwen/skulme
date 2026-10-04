pub fn cholesky(mat: Vec<f64>, n: usize) -> Vec<f64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (cholesky)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement cholesky");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cholesky() {
        // Test case 1
        let mat1 = vec![25.0, 15.0, -5.0, 15.0, 18.0, 0.0, -5.0, 0.0, 11.0];
        let res1 = cholesky(mat1, 3);

        // The expected Cholesky decomposition values
        #[allow(clippy::useless_vec)]
        let expected1 = vec![5.0, 0.0, 0.0, 3.0, 3.0, 0.0, -1.0, 1.0, 3.0];

        assert!(res1
            .iter()
            .zip(expected1.iter())
            .all(|(a, b)| (a - b).abs() < 1e-6));
    }

    fn transpose_matrix(mat: &[f64], n: usize) -> Vec<f64> {
        (0..n)
            .flat_map(|i| (0..n).map(move |j| mat[j * n + i]))
            .collect()
    }

    fn matrix_multiply(mat1: &[f64], mat2: &[f64], n: usize) -> Vec<f64> {
        (0..n)
            .flat_map(|i| {
                (0..n).map(move |j| {
                    (0..n).fold(0.0, |acc, k| acc + mat1[i * n + k] * mat2[k * n + j])
                })
            })
            .collect()
    }

    #[test]
    fn test_matrix_operations() {
        // Test case 1: Transposition
        let mat1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let transposed_mat1 = transpose_matrix(&mat1, 3);
        let expected_transposed_mat1 = vec![1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0];
        assert_eq!(transposed_mat1, expected_transposed_mat1);

        // Test case 2: Matrix multiplication
        let mat2 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let mat3 = vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let multiplied_mat = matrix_multiply(&mat2, &mat3, 3);
        let expected_multiplied_mat = vec![30.0, 24.0, 18.0, 84.0, 69.0, 54.0, 138.0, 114.0, 90.0];
        assert_eq!(multiplied_mat, expected_multiplied_mat);
    }

    #[test]
    fn empty_matrix() {
        let mat = vec![];
        let res = cholesky(mat, 0);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn matrix_with_all_zeros() {
        let mat3 = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let res3 = cholesky(mat3, 3);
        let expected3 = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(res3, expected3);
    }
}
