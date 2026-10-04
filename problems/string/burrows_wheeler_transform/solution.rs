pub fn burrows_wheeler_transform(input: &str) -> (String, usize) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (burrows_wheeler_transform)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement burrows_wheeler_transform");
}

pub fn inv_burrows_wheeler_transform<T: AsRef<str>>(input: (T, usize)) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (inv_burrows_wheeler_transform)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement inv_burrows_wheeler_transform");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //Ensure function stand-alone legitimacy
    fn stand_alone_function() {
        assert_eq!(
            burrows_wheeler_transform("CARROT"),
            ("CTRRAO".to_owned(), 1usize)
        );
        assert_eq!(inv_burrows_wheeler_transform(("CTRRAO", 1usize)), "CARROT");
        assert_eq!(
            burrows_wheeler_transform("THEALGORITHMS"),
            ("EHLTTRAHGOMSI".to_owned(), 11usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform(("EHLTTRAHGOMSI".to_string(), 11usize)),
            "THEALGORITHMS"
        );
        assert_eq!(
            burrows_wheeler_transform("!.!.!??.=::"),
            (":..!!?:=.?!".to_owned(), 0usize)
        );
        assert_eq!(
            inv_burrows_wheeler_transform((":..!!?:=.?!", 0usize)),
            "!.!.!??.=::"
        );
    }
    #[test]
    fn basic_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("CARROT")),
            "CARROT"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("TOMATO")),
            "TOMATO"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THISISATEST")),
            "THISISATEST"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("THEALGORITHMS")),
            "THEALGORITHMS"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("RUST")),
            "RUST"
        );
    }

    #[test]
    fn special_characters() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!.!.!??.=::")),
            "!.!.!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("!{}{}(((&&%%!??.=::")),
            "!{}{}(((&&%%!??.=::"
        );
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("//&$[]")),
            "//&$[]"
        );
    }

    #[test]
    fn empty() {
        assert_eq!(
            inv_burrows_wheeler_transform(burrows_wheeler_transform("")),
            ""
        );
    }
}
