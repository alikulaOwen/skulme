// Automatically generated category module

#[path = "blake2b/solution.rs"]
pub mod blake2b;

#[path = "fletcher/solution.rs"]
pub mod fletcher;

#[path = "hashing_traits/solution.rs"]
pub mod hashing_traits;

#[path = "md5/solution.rs"]
pub mod md5;

#[path = "sha1/solution.rs"]
pub mod sha1;

#[path = "sha2/solution.rs"]
pub mod sha2;

#[path = "sha3/solution.rs"]
pub mod sha3;


pub use self::blake2b::blake2b;
pub use self::fletcher::fletcher;
pub use self::hashing_traits::{Hasher, HMAC};
pub use self::md5::{md5, md5_hex};
pub use self::sha1::sha1;
pub use self::sha2::{sha224, sha256, sha384, sha512, sha512_224, sha512_256};
pub use self::sha3::{sha3_224, sha3_256, sha3_384, sha3_512};
