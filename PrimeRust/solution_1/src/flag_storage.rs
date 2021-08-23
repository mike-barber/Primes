/// Trait defining the interface to different kinds of storage, e.g.
/// bits within bytes, a vector of bytes, etc.
pub trait FlagStorage {
    /// create new storage for given number of flags pre-initialised to all true
    fn create_true(size: usize) -> Self;

    /// reset all flags for the given `skip` factor (prime)
    fn reset_flags(&mut self, skip: usize);

    /// get a specific flag
    fn get(&self, index: usize) -> bool;
}
