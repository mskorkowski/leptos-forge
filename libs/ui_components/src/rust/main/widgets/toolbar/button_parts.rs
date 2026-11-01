//! Module contains implementation of the parts of the buttons to be shown in
//! on the toolbar icon and label.

use leptos::prelude::component;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoAny;
use leptos::prelude::IntoView;
use leptos::prelude::view;

use crate::widgets::toolbar::ToolbarItemView;

use super::CSS_CLASSES_GENERIC_ITEM_LABEL;
use super::CSS_CLASSES_GENERIC_ITEM_ICON;

/// Label of the generic button in the toolbar
#[component]
pub fn Label<Item>(
    /// Value of the signal to be returned on button click
    item: Item,
) -> impl IntoView
where
    Item: ToolbarItemView
{
    let label = item.label().into_any();

    view!{
        <div class=CSS_CLASSES_GENERIC_ITEM_LABEL>
            { label }
        </div>
    }
}

/// Icon of the generic button in the toolbar
#[component]
pub fn Icon<Item>(
    /// Value of the signal to be returned on button click
    item: Item,
) -> impl IntoView 
where
    Item: ToolbarItemView,
{
    let icon = item.icon().into_any();

    view!{
        <div class=CSS_CLASSES_GENERIC_ITEM_ICON>
            { icon }
        </div>
    }
}