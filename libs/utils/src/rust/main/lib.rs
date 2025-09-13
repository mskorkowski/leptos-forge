//! Various custom basic data structures which are used around the whole system
#![doc(test(attr(deny(unused))))]
#![doc(test(attr(deny(dead_code))))]

#[cfg(feature="proptest")]
pub mod proptest;

mod immutable;
mod threadsafe;


/// Prelude to be imported for ease of use
pub mod prelude {
    pub use super::immutable::Immutable;

    pub use super::threadsafe::ThreadSafe;
}