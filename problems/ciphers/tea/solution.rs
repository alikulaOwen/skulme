use std::num::Wrapping as W;

struct TeaContext {
    key0: u64,
    key1: u64,
}

impl TeaContext {
    pub fn new(key: &[u64; 2]) -> TeaContext {
        TeaContext {
            key0: key[0],
            key1: key[1],
        }
    }

    pub fn encrypt_block(&self, block: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (encrypt_block)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement encrypt_block");
}

    pub fn decrypt_block(&self, block: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decrypt_block)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decrypt_block");
}
}

#[inline]
fn divide_u64(n: u64) -> (W<u32>, W<u32>) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (divide_u64)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement divide_u64");
}

pub fn tea_encrypt(plain: &[u8], key: &[u8]) -> Vec<u8> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (tea_encrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement tea_encrypt");
}

pub fn tea_decrypt(cipher: &[u8], key: &[u8]) -> Vec<u8> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (tea_decrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement tea_decrypt");
}

#[inline]
fn to_block(data: &[u8]) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (to_block)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement to_block");
}

fn from_block(block: u64) -> [u8; 8] {
    [
        block as u8,
        (block >> 8) as u8,
        (block >> 16) as u8,
        (block >> 24) as u8,
        (block >> 32) as u8,
        (block >> 40) as u8,
        (block >> 48) as u8,
        (block >> 56) as u8,
    ]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_block_convert() {
        assert_eq!(
            to_block(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]),
            0xefcdab8967452301
        );

        assert_eq!(
            from_block(0xefcdab8967452301),
            [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef]
        );
    }

    #[test]
    fn test_tea_encrypt() {
        assert_eq!(
            tea_encrypt(
                &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                &[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00
                ]
            ),
            [0x0A, 0x3A, 0xEA, 0x41, 0x40, 0xA9, 0xBA, 0x94]
        );
    }

    #[test]
    fn test_tea_encdec() {
        let plain = &[0x1b, 0xcc, 0xd4, 0x31, 0xa0, 0xf6, 0x8a, 0x55];
        let key = &[
            0x20, 0x45, 0x08, 0x10, 0xb0, 0x23, 0xe2, 0x17, 0xc3, 0x81, 0xd6, 0xf2, 0xee, 0x00,
            0xa4, 0x8a,
        ];
        let cipher = tea_encrypt(plain, key);

        assert_eq!(tea_decrypt(&cipher[..], key), plain);
    }
}
