//! Various data structures which are used by the widgets to represent some kind of data

use std::fmt::Debug;
use std::fmt::Display;

use uuid::Uuid;



/// Password
/// 
/// If you try to print or debug value it will print `********`
#[derive(Clone)]
pub struct Password(String);

impl Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Password").field(&"********").finish()
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("********")
    }
}

impl Password {
    /// Creates a new instance of [Password] with given value
    pub fn new(password: String) -> Self {
        Self(password)
    }

    /// Returns raw password value from the password struct
    /// 
    /// # Safety
    /// 
    /// This method is marked as unsafe because you are responsible to not leak the clear text password
    /// in irresponsible ways
    #[allow(unsafe_code)]
    pub unsafe fn get_raw_password_value(&self) -> &str {
        &self.0
    }

    /// Updates a password with a new value
    /// 
    /// # Safety
    /// 
    /// This methods is marked as unsafe since you are responsible for validating the password before
    /// updating this instance of Passowrd
    #[allow(unsafe_code)]
    pub unsafe fn set_raw_password_value(&mut self, password: String) {
        self.0 = password;
    }
}

/// Trait for items that have an unique identifier
/// 
/// This trait is used to provide a way to uniquely identify items in a collection so the lists of items
/// in the ui can be updated efficiently.
/// 
/// It used for example by
/// 
/// - [`SingleSelect`][crate::widgets::select::SingleSelect] so it can track the selected item
pub trait Keyed {
    /// Returns a sable key for this instance
    fn key(&self) -> &Uuid;
}