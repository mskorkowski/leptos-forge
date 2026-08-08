//! Password model

use std::fmt::Debug;
use std::fmt::Display;

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