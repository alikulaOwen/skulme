// https://en.wikipedia.org/wiki/Move-to-front_transform

fn blank_char_table() -> Vec<char> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (blank_char_table)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement blank_char_table");
}

pub fn move_to_front_encode(text: &str) -> Vec<u8> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (move_to_front_encode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement move_to_front_encode");
}

pub fn move_to_front_decode(encoded: &[u8]) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (move_to_front_decode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement move_to_front_decode");
}


#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_mtf {
        ($($name:ident: ($text:expr, $encoded:expr),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(move_to_front_encode($text), $encoded);
                    assert_eq!(move_to_front_decode($encoded), $text);
                }
            )*
        }
    }

    test_mtf! {
        empty: ("", &[]),
        single_char: ("@", &[64]),
        repeated_chars: ("aaba", &[97, 0, 98, 1]),
        mixed_chars: ("aZ!", &[97, 91, 35]),
        word: ("banana", &[98, 98, 110, 1, 1, 1]),
        special_chars: ("\0\n\t", &[0, 10, 10]),
    }
}
