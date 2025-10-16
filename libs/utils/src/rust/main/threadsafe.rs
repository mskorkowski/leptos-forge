//! Defines an alias for the `'static + Send + Sync` combination
//!

/// Type alias for `'static + Send + Sync` which is usable in generic type boundaries
pub trait ThreadSafe: 'static + Send + Sync {}

/// Carpet implementation of `ThreadSafe` trait
impl<T: 'static + Send + Sync> ThreadSafe for T {}
