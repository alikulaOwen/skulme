// returns the day of the week from the Gregorian Date

pub fn zellers_congruence_algorithm(date: i32, month: i32, year: i32, as_string: bool) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (zellers_congruence_algorithm)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement zellers_congruence_algorithm");
}

fn number_to_day(number: i32) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (number_to_day)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement number_to_day");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(zellers_congruence_algorithm(25, 1, 2013, false), "6");
        assert_eq!(zellers_congruence_algorithm(25, 1, 2013, true), "Friday");
        assert_eq!(zellers_congruence_algorithm(16, 4, 2022, false), "0");
        assert_eq!(zellers_congruence_algorithm(16, 4, 2022, true), "Saturday");
        assert_eq!(zellers_congruence_algorithm(14, 12, 1978, false), "5");
        assert_eq!(zellers_congruence_algorithm(15, 6, 2021, false), "3");
    }
}
