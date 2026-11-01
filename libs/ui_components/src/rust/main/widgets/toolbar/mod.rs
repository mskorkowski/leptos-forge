//! Mobile toolbar
//! 
//! Toolbar for mobile view inspired by the liquid glass look
//! 
//! 
mod button_parts;
mod buttons;

use std::marker::PhantomData;

use leptos::prelude::*;
use utils::prelude::Enumerable;
use utils::prelude::ThreadSafe;

use crate::model::ButtonClick;
use crate::model::ButtonColorSchema;
use crate::model::Kind;
use crate::model::Size;
use crate::model::Style;
use crate::widgets::button::Button;

pub use button_parts::*;
pub use buttons::*;

/// Provides the view specific information for the toolbar
/// 
/// - Icon will be shown above the label
/// - Icon will be centered above the text
/// - Menu options will be evenly distributed in the menu
/// 
/// # Tips
/// 
/// 1. You should prefer providing an icon over the label
/// 2. If you provide an icon you should return `None` from the  
///    [label][ToolbarItemView::label] to enhance the clarity of the menu
/// 3. If you provide the label you should return `None` from 
///    [icon][ToolbarItemView::icon]
/// 4. Provided icon should be readable and easy to understand
pub trait ToolbarItemView {
    /// Provides icon for the menu item
    fn icon(&self) -> Option<AnyView>;
    /// Provides label for the menu item
    fn label(&self) -> Option<AnyView>;
    /// Returns the element id for the toolbar componen
    fn id(&self) -> impl ToString;
}



/// CSS classes for toolbar
const CSS_CLASSES_TOOLBAR: &str = "\
    [--forge-toolbar-padding:calc(var(--spacing)_*_3)] \
    [--forge-toolbar-padding-button-single-active:calc(var(--spacing)_*_5)] \
    [--forge-toolbar-border:2px] \
    [--forge-toolbar-border-color-light:color-mix(in_oklab,_var(--color-forgegray-700)_20%,_transparent)] \
    [--forge-toolbar-border-color-dark:color-mix(in_oklab,_var(--color-forgegray-300)_20%,_transparent)] \
    [--forge-toolbar-bg-light:color-mix(in_oklab,_var(--color-forgegray-900)_5%,_transparent)] \
    [--forge-toolbar-bg-dark:color-mix(in_oklab,_var(--color-forgegray-100)_5%,_transparent)] \
    [--forge-toolbar-text-light:var(--color-forgegray-700)] \
    [--forge-toolbar-text-dark:var(--color-forgegray-300)] \
    [--forge-toolbar-text-light-hover:var(--color-forgeblue-600)] \
    [--forge-toolbar-text-dark-hover:var(--color-forgeblue-500)] \
    [--forge-toolbar-margin-default:calc(var(--spacing)_*4)] \
    [--forge-toolbar-margin-single-button-active:calc(var(--spacing)_*2)] \
    [--forge-toolbar-button-margin:calc(var(--spacing)_*2)] \
    text-(--forge-toolbar-text-light) \
    dark:text-(--forge-toolbar-text-dark) \
    flex \
    flex-nowrap \
    flex-row \
    content-center \
    p-0 \
    select-none \
";

/// CSS classes for the static separator
const CSS_CLASSES_STATIC_SEPARATOR: &str = "\
    min-w-3 \
    grow-1 \
    shrink-1 \
";

/// Button color schema for the search button
/// 
/// # Note
/// 
/// `active:` must duplicate the `hover:` rules because on mobile there is no
/// `hover` but we still need to provide an indicator of activation
const SINGLE_BUTTON_COLOR_SCHEME: ButtonColorSchema = ButtonColorSchema {
    inner: "\
        p-(--forge-toolbar-padding) \
        hover:text-(--forge-toolbar-text-light-hover) \
        hover:dark:text-(--forge-toolbar-text-dark-hover) \
        active:text-(--forge-toolbar-text-light-hover) \
        active:dark:text-(--forge-toolbar-text-dark-hover) \
        text-3xl \
    ",
    outer: "\
        inline-block \
        scale-100 \
        mx-(--forge-toolbar-margin-default) \
        active:scale-120 \
        place-self-center \
        [transition:background-color_150ms_ease,_text_150ms_ease,_border_150ms_ease,_color_150ms_ease,_scale_200ms_linear] \
        origin-center \
    ",
    border: "\
        border-(length:--forge-toolbar-border) \
        border-(--forge-toolbar-border-color-light) \
        dark:border-(--forge-toolbar-border-color-dark) \
        hover:border-(--forge-toolbar-text-light-hover) \
        hover:dark:border-(--forge-toolbar-text-dark-hover) \
        active:border-(--forge-toolbar-text-light-hover) \
        active:dark:border-(--forge-toolbar-text-dark-hover) \
        rounded-full \
    ",
    color: "",
    background: "\
        bg-(--forge-toolbar-bg-light) \
        dark:bg-(--forge-toolbar-bg-dark) \
        backdrop-blur-sm \
    ",
};

/// Style of single buttons
const SINGLE_BUTTON_STYLE: Style = Style{
    kind: Kind::Secondary,
    schema: SINGLE_BUTTON_COLOR_SCHEME,
    size: Size::Unsized,
};

/// CSS classes for generic item
const CSS_CLASSES_GENERIC_ITEM_CONTAINER: &str = "\
    border-(length:--forge-toolbar-border) \
    border-(--forge-toolbar-border-color-light) \
    dark:border-(--forge-toolbar-border-color-dark) \
    m-(--forge-toolbar-margin-default) \
    rounded-full \
    bg-(--forge-toolbar-bg-light) \
    dark:bg-(--forge-toolbar-bg-dark) \
    backdrop-blur-sm \
    flex \
    gap-3 \
    flex-nowrap \
    flex-row \
    place-items-center \
";

/// CSS for the generic item in the toolbar
const CSS_CLASSES_GENERIC_ITEM: &str = "\
    flex \
    flex-col \
    place-items-center \
    p-(--forge-toolbar-padding) \
    hover:text-(--forge-toolbar-text-light-hover) \
    hover:dark:text-(--forge-toolbar-text-dark-hover) \
    rounded-full \
";

/// CSS classes for the generic item icon
const CSS_CLASSES_GENERIC_ITEM_ICON: &str = "\
    text-3xl \
    md:text-base \
    lg:text-3xl \
";

/// CSS classes for the generic item label
const CSS_CLASSES_GENERIC_ITEM_LABEL: &str = "\
    sr-only \
    text-xs \
    lg:text-2xl \
    md:not-sr-only \
    md:block \
";

/// Button color schema for the buttons inside a toolbar
/// 
/// # Note
/// 
/// `active:` must duplicate the `hover:` rules because on mobile there is no
/// `hover` but we still need to provide an indicator of activation
const TOOLBAR_BUTTON_COLOR_SCHEME: ButtonColorSchema = ButtonColorSchema {
    inner: "\
        flex \
        flex-col \
        lg:flex-row \
        lg:gap-1 \
        place-items-center \
        py-(--forge-toolbar-padding) \
        pr-(--forge-toolbar-padding) \
        first:pl-(--forge-toolbar-padding) \
        hover:text-(--forge-toolbar-text-light-hover) \
        hover:dark:text-(--forge-toolbar-text-dark-hover) \
        active:text-(--forge-toolbar-text-light-hover) \
        active:dark:text-(--forge-toolbar-text-dark-hover) \
        text-3xl \
    ",
    // m-(--forge-toolbar-margin-default) \
    // active:m-(--forge-toolbar-margin-single-button-active) \
    outer: "\
        scale-100 \
        last:mr-(--forge-toolbar-button-margin) \
        first:ml-(--forge-toolbar-button-margin) \
        lg:mr-(--forge-toolbar-button-margin) \
        active:scale-120 \
        place-self-center \
        [transition:background-color_150ms_ease,_text_150ms_ease,_border_150ms_ease,_color_150ms_ease,_scale_200ms_linear] \
        origin-center \
    ",
    // border: "\
    //     border-(length:--forge-toolbar-border) \
    //     border-(--forge-toolbar-border-color-light) \
    //     dark:border-(--forge-toolbar-border-color-dark) \
    //     hover:border-(--forge-toolbar-text-light-hover) \
    //     hover:dark:border-(--forge-toolbar-text-dark-hover) \
    //     active:border-(--forge-toolbar-text-light-hover) \
    //     active:dark:border-(--forge-toolbar-text-dark-hover) \
    //     rounded-full \
    // ",
    border: "\
        rounded-full \
    ",
    color: "",
    // background: "\
    //     hover:bg-(--forge-toolbar-bg-light) \
    //     hover:dark:bg-(--forge-toolbar-bg-dark) \
    //     hover:backdrop-blur-sm \
    //     active:bg-(--forge-toolbar-bg-light) \
    //     active:dark:bg-(--forge-toolbar-bg-dark) \
    //     active:backdrop-blur-sm \
    // ",
    background: "\
    ",
};

/// Style of toolbar buttons
const TOOLBAR_BUTTON_STYLE: Style = Style{
    kind: Kind::Secondary,
    schema: TOOLBAR_BUTTON_COLOR_SCHEME,
    size: Size::Unsized,
};

/// Mobile toolbar
/// 
/// # Type arguments
/// 
/// - **Items** - type which can be enumerated 
/// 
/// # Arguments
/// 
/// - **search** - Value of the signal when the search button was selected.
///   Setting up this value will draw the search button with magnifying glass
///   to the right of the toolbar as the visually separate entity
#[component]
pub fn Toolbar<Items>(
    /// Items to show in the menu
    /// 
    /// The `Enumerate` implementation should only return values which should
    /// be added to the bottom toolbar.
    /// 
    /// ```rust
    /// # use leptos::prelude::*;
    /// 
    /// enum BottomNavigationMenu {
    ///     Home,
    ///     Tasks,
    ///     Options,
    ///     Search
    /// }
    /// 
    /// impl Enumerable {
    ///     fn iter() -> impl Iterator<Item = Self> {
    ///         use BottomNavigationMenu::*;
    ///         vec![
    ///             Home,
    ///             Tasks,
    ///             Options,
    ///                      /// Notice there is lack of Search here
    ///         ].into_iter()
    ///     }
    /// 
    /// }
    /// 
    /// # impl ToolbarItemView for BottomNavigationMenu {
    /// #   fn icon(&self) -> impl IntoView {
    /// #     view!{
    /// #        "icon"
    /// #     }    
    /// #   }
    /// #
    /// #   fn label(&self) -> impl  IntoView {
    /// #     use BottomNavigationMenu::*;
    /// #     let label = match self {
    /// #        Home => "Home",
    /// #        Tasks => "Tasks",
    /// #        Options => "Options",
    /// #        _ => ""
    /// #     }
    /// #   }
    /// # } 
    /// 
    /// view!{
    ///     <Toolbar<BottomNavigationMenu> 
    ///         search=BottomNavigationMenu.Search
    ///     />  
    /// }
    /// ```
    #[prop(optional, default=PhantomData)]
    _items: PhantomData<*const Items>,
    /// Item which should be selected when search option is 
    #[prop(optional)]
    search: Option<Items>,
    /// Signal set when user selects option on the toolbar
    #[prop(optional)]
    selection: Option<SignalSetter<Items>>,
) -> impl IntoView 
where
    Items: Enumerable + ThreadSafe + ToolbarItemView + Copy
{

    view!{
        <ToolbarContainer>
            <div class=CSS_CLASSES_GENERIC_ITEM_CONTAINER>
                {
                    Items::iter().map(|i| {
                        view!{
                            <BasicMenuItem 
                                item=i 
                                state=selection
                            />
                        }
                    }).collect_view()
                }
            </div>
            {
                match search {
                    Some(search)  => {
                        view!{
                            <Separator />
                            <SearchMenuItem 
                                item=search
                                state=selection
                            />
                        }.into_any()
                    },
                    None => {
                        ().into_any()
                    }
                }
            }
        </ToolbarContainer>
    }
}

/// Container holding the toolbar
/// 
/// This is internal component of the toolbar, used to setup the environment for
/// all other toolbar components.
#[component]
pub fn ToolbarContainer(
    /// Content of the toolbar container
    children: Children
) -> impl IntoView {
    view!{
        <div class=CSS_CLASSES_TOOLBAR>
            { children() }
        </div>
    }
}

/// Menu item for classic button like menu item
#[component]
pub fn BasicMenuItem<Item>(
    /// Menu item to be shown
    item: Item,
    /// signal with the toolbar state
    state: Option<SignalSetter<Item>>,
) -> impl IntoView 
where
    Item: ThreadSafe + Copy + ToolbarItemView,
{
    let (click_out, click_in) = signal(ButtonClick::Released);

    Effect::watch(
        move || click_out.get(), 
        move |value, _prev, _| {
            use ButtonClick::*;
            if value == &LeftClick
                && let Some(state) = state {
                    state.set(item);
                }
        }, 
        false
    );

    view!{
        <Button<String>
            id={item.id().to_string()}
            style=TOOLBAR_BUTTON_STYLE
            click=click_in
        >
            <Icon item />
            <Label item />
        </Button<String>>
    }
}

/// Search button menu item
/// 
/// This component depends on the [ToolbarContainer] to provide the html/css
/// environment
#[component]
pub fn SearchMenuItem<Item>(
    /// Value of the signal when menu item has been triggered
    item: Item,
    /// signal with the toolbar state
    state: Option<SignalSetter<Item>>,
) -> impl IntoView 
where
    Item: ThreadSafe + Copy + ToolbarItemView,
{

    let (click_out, click_in) = signal(ButtonClick::Released);

    Effect::watch(
        move || click_out.get(), 
        move |value, _prev, _| {
            use ButtonClick::*;
            if value == &LeftClick
                && let Some(state) = state {
                    state.set(item);
                }
        }, 
        false
    );

    view!{
        <Button<String>
            click=click_in
            style=SINGLE_BUTTON_STYLE
        >
            <leptos_icons::Icon icon=icondata::MdiMagnify />
        </Button<String>>
    }
}

/// Static minimum size separator
#[component]
pub fn Separator() -> impl IntoView {
    view!{
        <div class=CSS_CLASSES_STATIC_SEPARATOR />
    }
}