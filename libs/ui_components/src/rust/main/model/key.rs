//! All the structure and traits related to providing the key value for an
//! item in the collection so [stores][reactive_stores::Store] can wark


use std::fmt::Debug;

use reactive_stores::PatchField;
use reactive_stores::Store;
use uuid::Uuid;



/// Key for collections of values
#[derive(Debug, Clone, Copy, Store, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key(Uuid);

impl Key {
    /// Generates random key
    pub fn random() -> Self {
        Key(Uuid::new_v4())
    }

    /// Creates a key uuid from value
    pub const fn new(uuid: Uuid) -> Self{
        Key(uuid)
    }
}

impl From<Uuid> for Key {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}



impl PatchField for Key{
    fn patch_field(
        &mut self,
        new: Self,
        path: &reactive_stores::StorePath,
        notify: &mut dyn FnMut(&reactive_stores::StorePath),
        _keys: Option<&reactive_stores::KeyMap>,
    ) {
        if *self != new{
            *self = new;
            notify(path);
        }
    }
}



/// Trait for items that have an unique identifier
///
/// This trait is used to provide a way to uniquely identify items in a collection so the lists of items
/// in the ui can be updated efficiently.
///
/// It used for example by
///
/// - [`SingleSelect`][crate::widgets::single_select::SingleSelect] so it can track the selected item
pub trait Keyed {
    /// Returns a sable key for this instance
    fn key(&self) -> &Key;
}