use std::collections::HashMap;
use std::io;

const UNKNOWN_CHARACTER: &str = "........";
const _UNKNOWN_MORSE_CHARACTER: &str = "_";

pub fn encode(message: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (encode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement encode");
}

// Declarative macro for creating readable map declarations, for more info see https://doc.rust-lang.org/book/ch19-06-macros.html
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        std::iter::Iterator::collect(IntoIterator::into_iter([$(($key, $value),)*]))
    };
}

fn _morse_dictionary() -> HashMap<&'static str, &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_morse_dictionary)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _morse_dictionary");
}

fn _morse_to_alphanumeric_dictionary() -> HashMap<&'static str, &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_morse_to_alphanumeric_dictionary)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _morse_to_alphanumeric_dictionary");
}

fn _check_part(string: &str) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_check_part)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _check_part");
}

fn _check_all_parts(string: &str) -> bool {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_check_all_parts)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _check_all_parts");
}

fn _decode_token(string: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_decode_token)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _decode_token");
}

fn _decode_part(string: &str) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (_decode_part)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement _decode_part");
}

/// Convert morse code to ascii.
///
/// Given a morse code, return the corresponding message.
/// If the code is invalid, the undecipherable part of the code is replaced by `_`.
pub fn decode(string: &str) -> Result<String, io::Error> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decode)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decode");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_only_letters() {
        let message = "Hello Morse";
        let cipher = encode(message);
        assert_eq!(
            cipher,
            ".... . .-.. .-.. --- / -- --- .-. ... .".to_string()
        )
    }

    #[test]
    fn encrypt_letters_and_special_characters() {
        let message = "What's a great day!";
        let cipher = encode(message);
        assert_eq!(
            cipher,
            ".-- .... .- - .----. ... / .- / --. .-. . .- - / -.. .- -.-- -.-.--".to_string()
        )
    }

    #[test]
    fn encrypt_message_with_unsupported_character() {
        let message = "Error?? {}";
        let cipher = encode(message);
        assert_eq!(
            cipher,
            ". .-. .-. --- .-. ..--.. ..--.. / ........ ........".to_string()
        )
    }

    #[test]
    fn decrypt_valid_morsecode_with_spaces() {
        let expected = "Hello Morse! How's it goin, \"eh\"?"
            .to_string()
            .to_uppercase();
        let encypted = encode(&expected);
        let result = decode(&encypted).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn decrypt_valid_character_set_invalid_morsecode() {
        let expected = format!(
            "{_UNKNOWN_MORSE_CHARACTER}{_UNKNOWN_MORSE_CHARACTER}{_UNKNOWN_MORSE_CHARACTER}{_UNKNOWN_MORSE_CHARACTER} {_UNKNOWN_MORSE_CHARACTER}",
        );

        let encypted = ".-.-.--.-.-. --------. ..---.-.-. .-.-.--.-.-. / .-.-.--.-.-.".to_string();
        let result = decode(&encypted).unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn decrypt_invalid_morsecode_with_spaces() {
        let encypted = "1... . .-.. .-.. --- / -- --- .-. ... .";
        let result = decode(encypted).map_err(|e| e.kind());
        let expected = Err(io::ErrorKind::InvalidData);

        assert_eq!(expected, result);
    }
}
