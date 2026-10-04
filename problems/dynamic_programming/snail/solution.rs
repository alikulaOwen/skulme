/// ## Spiral Sorting
///
/// Given an n x m array, return the array elements arranged from outermost elements
/// to the middle element, traveling INWARD FROM TOP-LEFT, CLOCKWISE.
pub fn snail<T: Copy>(matrix: &[Vec<T>]) -> Vec<T> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (snail)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement snail");
}

enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    pub fn snail_move(
        &mut self,
        col: &mut usize,
        row: &mut usize,
        min_col: &mut usize,
        max_col: &mut usize,
        min_row: &mut usize,
        max_row: &mut usize,
    ) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (snail_move)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement snail_move");
}
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let empty: &[Vec<i32>] = &[vec![]];
        assert_eq!(snail(empty), vec![]);
    }

    #[test]
    fn test_int() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(snail(square), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_char() {
        let square = &[
            vec!['S', 'O', 'M'],
            vec!['E', 'T', 'H'],
            vec!['I', 'N', 'G'],
        ];
        assert_eq!(
            snail(square),
            vec!['S', 'O', 'M', 'H', 'G', 'N', 'I', 'E', 'T']
        );
    }

    #[test]
    fn test_rect() {
        let square = &[
            vec!['H', 'E', 'L', 'L'],
            vec!['O', ' ', 'W', 'O'],
            vec!['R', 'L', 'D', ' '],
        ];
        assert_eq!(
            snail(square),
            vec!['H', 'E', 'L', 'L', 'O', ' ', 'D', 'L', 'R', 'O', ' ', 'W']
        );
    }
}
