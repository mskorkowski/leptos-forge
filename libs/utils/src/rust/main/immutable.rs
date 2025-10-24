//! Immutable new type structure for storing data

use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::Add;
use std::ops::Deref;
use std::ops::Div;
use std::ops::Index;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

/// Immutable data structure
///
/// You can access the data using `&self` or `self.deref()` but there is no way to get mutable reference.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Immutable<T>(T);

impl<T> Deref for Immutable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for Immutable<T> {
    fn from(value: T) -> Self {
        Immutable(value)
    }
}

impl<T: Default> Default for Immutable<T> {
    fn default() -> Self {
        Immutable(T::default())
    }
}

impl<T: Clone> Clone for Immutable<T> {
    fn clone(&self) -> Self {
        Immutable(self.0.clone())
    }
}

impl<T: Copy> Copy for Immutable<T> {}

impl<T: Hash> Hash for Immutable<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: PartialOrd> PartialOrd for Immutable<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord> Ord for Immutable<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: PartialEq> PartialEq for Immutable<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: Eq> Eq for Immutable<T> {}

impl<T: Display> Display for Immutable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Debug> Debug for Immutable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Immutable").field(&self.0).finish()
    }
}

impl<T: Add<T, Output = T>> Add<T> for Immutable<T> {
    type Output = T;

    fn add(self, other: T) -> Self::Output {
        self.0 + other
    }
}

impl<T: Add<T, Output = T>> Add<Immutable<T>> for Immutable<T> {
    type Output = T;

    fn add(self, other: Immutable<T>) -> Self::Output {
        self.0 + other.0
    }
}

impl<T: Sub<T, Output = T>> Sub<T> for Immutable<T> {
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        self.0 - rhs
    }
}

impl<T: Sub<T, Output = T>> Sub<Immutable<T>> for Immutable<T> {
    type Output = T;

    fn sub(self, other: Immutable<T>) -> Self::Output {
        self.0 - other.0
    }
}

impl<T: Mul<T, Output = T>> Mul<T> for Immutable<T> {
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        self.0 * rhs
    }
}

impl<T: Mul<T, Output = T>> Mul<Immutable<T>> for Immutable<T> {
    type Output = T;

    fn mul(self, other: Immutable<T>) -> Self::Output {
        self.0 * other.0
    }
}

impl<T: Div<T, Output = T>> Div<T> for Immutable<T> {
    type Output = T;

    fn div(self, rhs: T) -> Self::Output {
        self.0 / rhs
    }
}

impl<T: Div<T, Output = T>> Div<Immutable<T>> for Immutable<T> {
    type Output = T;

    fn div(self, other: Immutable<T>) -> Self::Output {
        self.0 / other.0
    }
}

impl<T: Rem<T, Output = T>> Rem<T> for Immutable<T> {
    type Output = T;

    fn rem(self, rhs: T) -> Self::Output {
        self.0 % rhs
    }
}

#[allow(unsafe_code)]
/// Safety: Allows sending the immutable if and only if T is Send since we don't
/// introduce operations on T
unsafe impl<T: Send> Send for Immutable<T> {}

#[allow(unsafe_code)]
/// Safety: References can be shared across thread boundaries as long as T is Sync
/// since we don't introduce any operation on T
unsafe impl<T: Sync> Sync for Immutable<T> {}

impl<T> Borrow<T> for Immutable<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> AsRef<T> for Immutable<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T, I> Index<I> for Immutable<T>
where
    T: Index<I>,
{
    type Output = T::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> Immutable<T>
where
    T: Copy,
{
    /// Gets the copy of value in the container
    pub fn get(&self) -> T {
        self.0
    }
}
