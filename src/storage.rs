pub mod mem;

/// Basic key-value storage interface.
pub trait Storage {
    type Key;
    type Value;

    /// Get value by key.
    fn get(&self, key: &Self::Key) -> anyhow::Result<Option<Self::Value>>;

    /// Set value for the key.
    /// Existing value would be overwritten if present.
    ///
    /// Returns previous value if exists.
    fn set(&mut self, key: &Self::Key, value: Self::Value) -> anyhow::Result<Option<Self::Value>>;

    /// Delete value by key.
    ///
    /// Returns deleted value if exists.
    fn delete(&mut self, key: &Self::Key) -> anyhow::Result<Option<Self::Value>>;
}
