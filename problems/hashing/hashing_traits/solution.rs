pub trait Hasher<const DIGEST_BYTES: usize> {
    /// Return a new instance with default parameters.
    fn new_default() -> Self;

    /// Add new data.
    fn update(&mut self, data: &[u8]);

    /// Returns the hash of current data. If necessary does finalization work
    /// on the instance, thus it may no longer make sense to call `update`
    /// after calling this.
    fn get_hash(&mut self) -> [u8; DIGEST_BYTES];
}

/// HMAC based on RFC 2104, applicable to many cryptographic hash functions.
pub struct HMAC<const KEY_BYTES: usize, const DIGEST_BYTES: usize, H: Hasher<DIGEST_BYTES>> {
    pub inner_internal_state: H,
    pub outer_internal_state: H,
}

impl<const KEY_BYTES: usize, const DIGEST_BYTES: usize, H: Hasher<DIGEST_BYTES>>
    HMAC<KEY_BYTES, DIGEST_BYTES, H>
{
    pub fn new_default() -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new_default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new_default");
}

    /// Note that `key` must be no longer than `KEY_BYTES`. According to the
    /// RFC, if it is so, you should replace it with its hash. We do not do
    /// this automatically due to fear of `DIGEST_BYTES` not being the same as
    /// `KEY_BYTES` or even being longer than it.
    pub fn add_key(&mut self, key: &[u8]) -> Result<(), &'static str> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (add_key)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement add_key");
}

    pub fn update(&mut self, data: &[u8]) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (update)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement update");
}

    pub fn finalize(&mut self) -> [u8; DIGEST_BYTES] {
        self.outer_internal_state
            .update(&self.inner_internal_state.get_hash());
        self.outer_internal_state.get_hash()
    }
}

