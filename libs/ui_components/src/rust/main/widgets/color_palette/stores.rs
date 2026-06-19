//! Contains implementation for [reactive_stores] for data structures of the
//! [SingleSelect]
//! 

use reactive_stores::PatchField;

use crate::widgets::color_palette::ColorCode;


impl PatchField for ColorCode {
    fn patch_field(
            &mut self,
            new: Self,
            path: &reactive_stores::StorePath,
            notify: &mut dyn FnMut(&reactive_stores::StorePath),
            _: Option<&reactive_stores::KeyMap>,
    ) {
        if *self != new {
            *self = new;
            notify(path);
        }   
    }
}