use std::collections::VecDeque;

// All four potential movements from a cell are listed here.

fn validate(matrix: &[Vec<i32>], visited: &[Vec<bool>], row: isize, col: isize) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (validate)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement validate");
}

pub fn lee(matrix: Vec<Vec<i32>>, source: (usize, usize), destination: (usize, usize)) -> isize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (lee)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement lee");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lee_exists() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
        ];
        let source = (0, 0);
        let dest = (2, 1);
        assert_eq!(lee(mat, source, dest), 3);
    }

    #[test]
    fn test_lee_does_not_exist() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
        ];
        let source = (0, 0);
        let dest = (3, 4);
        assert_eq!(lee(mat, source, dest), -1);
    }

    #[test]
    fn test_source_equals_destination() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 0, 1, 1, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 0, 1],
            vec![1, 1, 1, 0, 1],
        ];
        let source = (2, 1);
        let dest = (2, 1);
        assert_eq!(lee(mat, source, dest), 0);
    }

    #[test]
    fn test_lee_exists_2() {
        let mat: Vec<Vec<i32>> = vec![
            vec![1, 1, 1, 1, 1, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 0, 1],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0],
        ];
        let source = (0, 0);
        let dest = (3, 2);
        assert_eq!(lee(mat, source, dest), 5);
    }
}
