//! Module contains the code of the menu

use leptos::prelude::*;

use leptos_node_ref::AnyNodeRef;

/// CSS classes of the main menu box
const MAIN_MENU_CSS_CLASSES: &str = "\
  leptos-forge-main-menu \
  flex-none \
  flex-row \
  md:flex-col \
  w-full \
  md:w-61 \
  min-h-10 \
  md:h-auto \
  py-0 \
  md:py-4 \
  px-0 \
  md:px-4 \
  flex \
  bg-forgeblue-950 \
  text-forgegray-200 \
  basis-auto \
  print:hidden \
  forge-text-standard
  md:scrollbox \
";

/// CSS Classes of the main menu container itself
const MAIN_MENU_CONTAINER_CSS_CLASSES: &str = "\
  flex \
  flex-row \
  justify-between \
  items-center \
  w-full \
  md:justify-normal
  md:flex-col \
  md:scrollable \
";

/// Vertical main menu component on the left hand side
/// 
/// # CSS Variables
/// 
/// This component defines a given list of css variables to be used by the
/// children
/// 
/// - **--forge-main-menu-width** - width of the main menu bar when seen on the
///   large screen
#[component]
pub fn MainMenu(
    /// Child elements of the main menu
    children: Children,
    /// Menu panel container
    #[prop(optional, default=AnyNodeRef::new())]
    container_ref: AnyNodeRef,
) -> impl IntoView {
    view! {
      <div 
        class=MAIN_MENU_CSS_CLASSES
        style=("--forge-main-menu-width", "calc(var(--spacing) * 61)")
      >
        <div
          node_ref=container_ref
          class=MAIN_MENU_CONTAINER_CSS_CLASSES
        >
          { children() }
        </div>
      </div>
    }
}
