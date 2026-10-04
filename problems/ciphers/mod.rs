// Automatically generated category module

#[path = "aes/solution.rs"]
pub mod aes;

#[path = "affine_cipher/solution.rs"]
pub mod affine_cipher;

#[path = "another_rot13/solution.rs"]
pub mod another_rot13;

#[path = "baconian_cipher/solution.rs"]
pub mod baconian_cipher;

#[path = "base16/solution.rs"]
pub mod base16;

#[path = "base32/solution.rs"]
pub mod base32;

#[path = "base64/solution.rs"]
pub mod base64;

#[path = "base85/solution.rs"]
pub mod base85;

#[path = "caesar/solution.rs"]
pub mod caesar;

#[path = "chacha/solution.rs"]
pub mod chacha;

#[path = "diffie_hellman/solution.rs"]
pub mod diffie_hellman;

#[path = "gronsfeld/solution.rs"]
pub mod gronsfeld;

#[path = "hill_cipher/solution.rs"]
pub mod hill_cipher;

#[path = "kernighan/solution.rs"]
pub mod kernighan;

#[path = "morse_code/solution.rs"]
pub mod morse_code;

#[path = "polybius/solution.rs"]
pub mod polybius;

#[path = "rail_fence/solution.rs"]
pub mod rail_fence;

#[path = "rot13/solution.rs"]
pub mod rot13;

#[path = "rsa_cipher/solution.rs"]
pub mod rsa_cipher;

#[path = "salsa/solution.rs"]
pub mod salsa;

#[path = "tea/solution.rs"]
pub mod tea;

#[path = "theoretical_rot13/solution.rs"]
pub mod theoretical_rot13;

#[path = "transposition/solution.rs"]
pub mod transposition;

#[path = "trifid/solution.rs"]
pub mod trifid;

#[path = "vernam/solution.rs"]
pub mod vernam;

#[path = "vigenere/solution.rs"]
pub mod vigenere;

#[path = "xor/solution.rs"]
pub mod xor;


pub use self::aes::{aes_decrypt, aes_encrypt, AesKey};
pub use self::affine_cipher::{affine_decrypt, affine_encrypt, affine_generate_key};
pub use self::another_rot13::another_rot13;
pub use self::baconian_cipher::{baconian_decode, baconian_encode};
pub use self::base16::{base16_decode, base16_encode};
pub use self::base32::{base32_decode, base32_encode};
pub use self::base64::{base64_decode, base64_encode};
pub use self::base85::{base85_decode, base85_encode};
pub use self::caesar::caesar;
pub use self::chacha::chacha20;
pub use self::diffie_hellman::DiffieHellman;
pub use self::gronsfeld::{gronsfeld_decrypt, gronsfeld_encrypt};
pub use self::hill_cipher::HillCipher;
pub use self::kernighan::kernighan;
pub use self::morse_code::{decode, encode};
pub use self::polybius::{decode_ascii, encode_ascii};
pub use self::rail_fence::{rail_fence_decrypt, rail_fence_encrypt};
pub use self::rot13::rot13;
pub use self::rsa_cipher::{
    decrypt, decrypt_text, encrypt, encrypt_text, generate_keypair, PrivateKey, PublicKey,
};
pub use self::salsa::salsa20;
pub use self::tea::{tea_decrypt, tea_encrypt};
pub use self::theoretical_rot13::theoretical_rot13;
pub use self::transposition::transposition;
pub use self::trifid::{trifid_decrypt, trifid_encrypt};
pub use self::vernam::{vernam_decrypt, vernam_encrypt};
pub use self::vigenere::vigenere;
pub use self::xor::xor;
