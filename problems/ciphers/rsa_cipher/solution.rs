//! RSA Cipher Implementation
//!
//! This module provides a basic implementation of the RSA (Rivest-Shamir-Adleman) encryption algorithm.
//! RSA is an asymmetric cryptographic algorithm that uses a pair of keys: public and private.
//!
//! # Warning
//!
//! This is an educational implementation and should NOT be used for production cryptography.
//! Use established cryptographic libraries like `ring` or `rust-crypto` for real-world applications.
//!
//! # Examples
//!
//! ```
//! use the_algorithms_rust::ciphers::{generate_keypair, encrypt, decrypt};
//!
//! let (public_key, private_key) = generate_keypair(61, 53);
//! let message = 65;
//! let encrypted = encrypt(message, &public_key);
//! let decrypted = decrypt(encrypted, &private_key);
//! assert_eq!(message, decrypted);
//! ```

/// Represents an RSA public key containing (n, e)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicKey {
    pub n: u64,
    pub e: u64,
}

/// Represents an RSA private key containing (n, d)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrivateKey {
    pub n: u64,
    pub d: u64,
}

/// Computes the greatest common divisor using Euclid's algorithm
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
///
/// The GCD of `a` and `b`
fn gcd(mut a: u64, mut b: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (gcd)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement gcd");
}

/// Computes the modular multiplicative inverse using the Extended Euclidean Algorithm
///
/// Finds `x` such that `(a * x) % m == 1`
///
/// # Arguments
///
/// * `a` - The number to find the inverse of
/// * `m` - The modulus
///
/// # Returns
///
/// The modular multiplicative inverse of `a` modulo `m`, or `None` if it doesn't exist
fn mod_inverse(a: i64, m: i64) -> Option<u64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mod_inverse)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mod_inverse");
}

/// Performs modular exponentiation: (base^exp) % modulus
///
/// Uses the square-and-multiply algorithm for efficiency
///
/// # Arguments
///
/// * `base` - The base number
/// * `exp` - The exponent
/// * `modulus` - The modulus
///
/// # Returns
///
/// The result of (base^exp) % modulus
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (mod_pow)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement mod_pow");
}

/// Generates an RSA keypair from two prime numbers
///
/// # Arguments
///
/// * `p` - First prime number
/// * `q` - Second prime number (should be different from p)
///
/// # Returns
///
/// A tuple containing (PublicKey, PrivateKey)
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::generate_keypair;
///
/// let (public, private) = generate_keypair(61, 53);
/// // n = p * q
/// assert_eq!(public.n, 3233);
/// assert_eq!(private.n, 3233);
/// // Both keys share the same n
/// assert_eq!(public.n, private.n);
/// ```
///
/// # Panics
///
/// Panics if the modular inverse cannot be computed
pub fn generate_keypair(p: u64, q: u64) -> (PublicKey, PrivateKey) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (generate_keypair)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement generate_keypair");
}

/// Encrypts a message using the RSA public key
///
/// # Arguments
///
/// * `message` - The plaintext message (must be less than n)
/// * `public_key` - The public key to use for encryption
///
/// # Returns
///
/// The encrypted ciphertext
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::{generate_keypair, encrypt, decrypt};
///
/// let (public_key, private_key) = generate_keypair(61, 53);
/// let message = 65;
/// let ciphertext = encrypt(message, &public_key);
/// let decrypted = decrypt(ciphertext, &private_key);
/// assert_eq!(decrypted, message);
/// ```
pub fn encrypt(message: u64, public_key: &PublicKey) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (encrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement encrypt");
}

/// Decrypts a ciphertext using the RSA private key
///
/// # Arguments
///
/// * `ciphertext` - The encrypted message
/// * `private_key` - The private key to use for decryption
///
/// # Returns
///
/// The decrypted plaintext message
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::{generate_keypair, encrypt, decrypt};
///
/// let (public_key, private_key) = generate_keypair(61, 53);
/// let message = 65;
/// let ciphertext = encrypt(message, &public_key);
/// let plaintext = decrypt(ciphertext, &private_key);
/// assert_eq!(plaintext, message);
/// ```
pub fn decrypt(ciphertext: u64, private_key: &PrivateKey) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decrypt)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decrypt");
}

/// Encrypts a text message by converting each character to its ASCII value
///
/// # Arguments
///
/// * `message` - The plaintext string
/// * `public_key` - The public key to use for encryption
///
/// # Returns
///
/// A vector of encrypted values, one for each character
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::{generate_keypair, encrypt_text, decrypt_text};
///
/// let (public, private) = generate_keypair(61, 53);
/// let encrypted = encrypt_text("HI", &public);
/// let decrypted = decrypt_text(&encrypted, &private);
/// assert_eq!(decrypted, "HI");
/// ```
pub fn encrypt_text(message: &str, public_key: &PublicKey) -> Vec<u64> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (encrypt_text)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement encrypt_text");
}

/// Decrypts a vector of encrypted values back to text
///
/// # Arguments
///
/// * `ciphertext` - The vector of encrypted character values
/// * `private_key` - The private key to use for decryption
///
/// # Returns
///
/// The decrypted string
///
/// # Examples
///
/// ```
/// use the_algorithms_rust::ciphers::{generate_keypair, encrypt_text, decrypt_text};
///
/// let (public, private) = generate_keypair(61, 53);
/// let encrypted = encrypt_text("HELLO", &public);
/// let decrypted = decrypt_text(&encrypted, &private);
/// assert_eq!(decrypted, "HELLO");
/// ```
pub fn decrypt_text(ciphertext: &[u64], private_key: &PrivateKey) -> String {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (decrypt_text)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement decrypt_text");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 50), 50);
        assert_eq!(gcd(7, 7), 7);
    }

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(17, 3120), Some(2753));
        assert_eq!(mod_inverse(7, 40), Some(23));
        assert!(mod_inverse(4, 8).is_none()); // No inverse exists
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(4, 13, 497), 445);
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 5, 7), 5);
    }

    #[test]
    fn test_generate_keypair() {
        let (public, private) = generate_keypair(61, 53);

        // n should be p * q
        assert_eq!(public.n, 3233);
        assert_eq!(private.n, 3233);

        // e should be coprime with phi
        let phi = (61 - 1) * (53 - 1);
        assert_eq!(gcd(public.e, phi), 1);

        // Verify that (e * d) % phi == 1
        assert_eq!((public.e * private.d) % phi, 1);
    }

    #[test]
    fn test_encrypt_decrypt_number() {
        let (public, private) = generate_keypair(61, 53);
        let message = 65;

        let encrypted = encrypt(message, &public);
        // Encrypted value will vary based on e, so we just check it's different
        assert_ne!(encrypted, message);

        let decrypted = decrypt(encrypted, &private);
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_encrypt_decrypt_various_numbers() {
        let (public, private) = generate_keypair(61, 53);

        for message in [1, 42, 100, 255, 1000, 3000] {
            let encrypted = encrypt(message, &public);
            let decrypted = decrypt(encrypted, &private);
            assert_eq!(decrypted, message, "Failed for message: {message}");
        }
    }

    #[test]
    fn test_encrypt_decrypt_text() {
        let (public, private) = generate_keypair(61, 53);

        let message = "HI";
        let encrypted = encrypt_text(message, &public);
        let decrypted = decrypt_text(&encrypted, &private);

        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_encrypt_decrypt_longer_text() {
        let (public, private) = generate_keypair(61, 53);

        let message = "HELLO";
        let encrypted = encrypt_text(message, &public);
        let decrypted = decrypt_text(&encrypted, &private);

        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_different_primes() {
        let (public, private) = generate_keypair(17, 19);

        let message = 42;
        let encrypted = encrypt(message, &public);
        let decrypted = decrypt(encrypted, &private);

        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_encrypt_decrypt_alphabet() {
        let (public, private) = generate_keypair(61, 53);

        let message = "ABC";
        let encrypted = encrypt_text(message, &public);
        let decrypted = decrypt_text(&encrypted, &private);

        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_key_properties() {
        let (public, private) = generate_keypair(61, 53);

        // Both keys should have the same n
        assert_eq!(public.n, private.n);

        // e and d should be different
        assert_ne!(public.e, private.d);

        // Verify that (e * d) % phi == 1
        let phi = (61 - 1) * (53 - 1);
        assert_eq!((public.e * private.d) % phi, 1);
    }
}
