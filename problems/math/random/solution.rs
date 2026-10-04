/*
Permuted Congruential Generator
https://en.wikipedia.org/wiki/Permuted_congruential_generator

Note that this is _NOT_ intended for serious applications. Use this generator
at your own risk and only use your own values instead of the default ones if
you really know what you are doing.
 */
pub struct PCG32 {
    state: u64,
    multiplier: u64,
    increment: u64,
}

pub const PCG32_MULTIPLIER: u64 = 6364136223846793005_u64;
pub const PCG32_INCREMENT: u64 = 1442695040888963407_u64;

pub struct IterMut<'a> {
    pcg: &'a mut PCG32,
}

impl PCG32 {
    /// `stream` should be less than 1 << 63
    pub fn new(seed: u64, multiplier: u64, stream: u64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new");
}
    pub fn new_default(seed: u64) -> Self {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (new_default)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement new_default");
}
    #[inline]
    pub fn next(&mut self) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
    #[inline]
    /// Advance the PCG by `delta` steps in O(lg(`delta`)) time. By passing
    /// a negative i64 as u64, it can go back too.
    pub fn advance(&mut self, mut delta: u64) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (advance)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement advance");
}
    #[inline]
    pub fn get_u32(&mut self) -> u32 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_u32)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_u32");
}
    #[inline]
    pub fn get_u64(&mut self) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_u64)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_u64");
}
    #[inline]
    pub fn get_u16(&mut self) -> (u16, u16) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_u16)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_u16");
}
    #[inline]
    pub fn get_u8(&mut self) -> (u8, u8, u8, u8) {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_u8)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_u8");
}
    #[inline]
    pub fn get_state(&self) -> u64 {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (get_state)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement get_state");
}
    pub fn iter_mut(&mut self) -> IterMut<'_> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (iter_mut)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement iter_mut");
}
}

impl Iterator for IterMut<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
    // =========================================================================
    // 🎯 YOUR MISSION: IMPLEMENT THIS METHOD (next)!
    // Follow the Socratic hints and questions in the comments above.
    // =========================================================================
    todo!("Implement next");
}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_birthday() {
        // If the distribution is not almost uniform, the probability of
        // birthday paradox increases. For n=2^32 and k=1e5, the probability
        // of not having a collision is about (1 - (k+1)/n) ^ (k/2) which is
        // 0.3121 for this (n, k).
        // So this test is a (dumb) test for distribution, and for speed. This
        // is only basic sanity checking, as the actual algorithm was
        // rigorously tested by others before.
        let numbers = 1e5 as usize;
        let mut pcg = PCG32::new_default(314159);
        let mut pcg2 = PCG32::new_default(314159);
        assert_eq!(pcg.get_u32(), pcg2.get_u32());
        let mut randoms: Vec<u32> = pcg.iter_mut().take(numbers).collect::<Vec<u32>>();
        pcg2.advance(1000);
        assert_eq!(pcg2.get_u32(), randoms[1000]);
        pcg2.advance((-1001_i64) as u64);
        assert_eq!(pcg2.get_u32(), randoms[0]);
        randoms.sort_unstable();
        randoms.dedup();
        assert_eq!(randoms.len(), numbers);
    }
}
