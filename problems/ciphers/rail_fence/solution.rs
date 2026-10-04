// wiki: https://en.wikipedia.org/wiki/Rail_fence_cipher
pub fn rail_fence_encrypt(plain_text: &str, key: usize) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rail_fence_encrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rail_fence_encrypt");
}

pub fn rail_fence_decrypt(cipher: &str, key: usize) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (rail_fence_decrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement rail_fence_decrypt");
}

fn zigzag(n: usize) -> impl Iterator<Item = usize> {
    #[allow(unreachable_code)]
    {
        todo!("Implement zigzag");
        std::iter::empty()
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rails_basic() {
        assert_eq!(rail_fence_encrypt("attack at once", 2), "atc toctaka ne");
        assert_eq!(rail_fence_decrypt("atc toctaka ne", 2), "attack at once");

        assert_eq!(rail_fence_encrypt("rust is cool", 3), "r cuti olsso");
        assert_eq!(rail_fence_decrypt("r cuti olsso", 3), "rust is cool");
    }
}
