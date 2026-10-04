// Author : cyrixninja
// Perfect Square : Checks if a number is perfect square number or not
// https://en.wikipedia.org/wiki/Perfect_square
pub fn perfect_square(num: i32) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (perfect_square)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement perfect_square");
}

pub fn perfect_square_binary_search(n: i32) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (perfect_square_binary_search)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement perfect_square_binary_search");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_square() {
        assert!(perfect_square(9));
        assert!(perfect_square(81));
        assert!(perfect_square(4));
        assert!(perfect_square(0));
        assert!(!perfect_square(3));
        assert!(!perfect_square(-19));
    }

    #[test]
    fn test_perfect_square_binary_search() {
        assert!(perfect_square_binary_search(9));
        assert!(perfect_square_binary_search(81));
        assert!(perfect_square_binary_search(4));
        assert!(perfect_square_binary_search(0));
        assert!(!perfect_square_binary_search(3));
        assert!(!perfect_square_binary_search(-19));
    }
}
