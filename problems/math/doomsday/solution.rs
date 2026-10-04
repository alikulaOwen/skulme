const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

pub fn doomsday(y: i32, m: i32, d: i32) -> i32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (doomsday)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement doomsday");
}

pub fn get_week_day(y: i32, m: i32, d: i32) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_week_day)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_week_day");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doomsday_test() {
        assert_eq!(get_week_day(1990, 3, 21), "Wednesday");
        assert_eq!(get_week_day(2000, 8, 24), "Thursday");
        assert_eq!(get_week_day(2000, 10, 13), "Friday");
        assert_eq!(get_week_day(2001, 4, 18), "Wednesday");
        assert_eq!(get_week_day(2002, 3, 19), "Tuesday");
    }
}
