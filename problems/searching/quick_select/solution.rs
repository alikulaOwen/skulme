// https://en.wikipedia.org/wiki/Quickselect

fn partition(list: &mut [i32], left: usize, right: usize, pivot_index: usize) -> usize {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (partition)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement partition");
}

pub fn quick_select(list: &mut [i32], left: usize, right: usize, index: usize) -> i32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (quick_select)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement quick_select");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut arr1 = [2, 3, 4, 5];
        assert_eq!(quick_select(&mut arr1, 0, 3, 1), 3);
        let mut arr2 = [2, 5, 9, 12, 16];
        assert_eq!(quick_select(&mut arr2, 1, 3, 2), 9);
        let mut arr2 = [0, 3, 8];
        assert_eq!(quick_select(&mut arr2, 0, 0, 0), 0);
    }
}
