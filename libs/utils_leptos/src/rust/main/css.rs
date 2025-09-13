//! Contains some useful utilities not covered by `use_leptos`
//! 

use std::fmt::Debug;

use leptos::leptos_dom::logging::console_log;
use leptos_use::core::ElementMaybeSignal;
use reactive_graph::traits::GetUntracked;
use leptos_use::core::IntoElementMaybeSignal;
use web_sys::DomTokenList;
use web_sys::Element;

/// Trait which allows a various types to be treated as a CSS class
/// 
/// Mostly for convenience so we can add and remove multiple classes in single step
pub trait ToClassDef: Debug {
    /// remove the classes from the element class list
    fn remove(self, class_list: &DomTokenList);
    /// add the classes to the element class list
    fn add(self, class_list: &DomTokenList);
}

impl ToClassDef for () {
    fn add(self, _class_list: &DomTokenList) {}
    fn remove(self, _class_list: &DomTokenList) {}
}

impl ToClassDef for &str {
    fn add(self, class_list: &DomTokenList) {
        let _ = class_list.add_1(self);
    }

    fn remove(self, class_list: &DomTokenList) {
        let _ = class_list.remove_1(self);
    }
}

impl ToClassDef for String {
    fn add(self, class_list: &DomTokenList) {
        let _ = class_list.add_1(&self);
    }

    fn remove(self, class_list: &DomTokenList) {
        let _ = class_list.remove_1(&self);
    }
}

impl ToClassDef for (&str, ) 
{
    fn add(self, class_list: &DomTokenList) {
        let _ = class_list.add_1(self.0);
    }

    fn remove(self, class_list: &DomTokenList) {
        let _ = class_list.remove_1(self.0);
    }
}

impl ToClassDef for (&str, &str) 
{
    fn add(self, class_list: &DomTokenList) {
        let _ = class_list.add_2(self.0, self.1);
    }

    fn remove(self, class_list: &DomTokenList) {
        let _ = class_list.remove_2(self.0, self.1);
    }
}

impl ToClassDef for (&str, &str, &str) 
{
    fn add(self, class_list: &DomTokenList) {
        let _ = class_list.add_3(self.0, self.1, self.2);
    }

    fn remove(self, class_list: &DomTokenList) {
        let _ = class_list.remove_3(self.0, self.1, self.2);
    }
}

impl ToClassDef for (&str, &str, &str, &str) 
{
    fn add(self, class_list: &DomTokenList) {
        let _ = class_list.add_4(self.0, self.1, self.2, self.3);
    }

    fn remove(self, class_list: &DomTokenList) {
        let _ = class_list.remove_4(self.0, self.1, self.2, self.3);
    }
}

impl ToClassDef for (&str, &str, &str, &str, &str) 
{
    fn add(self, class_list: &DomTokenList) {
        let _ = class_list.add_5(self.0, self.1, self.2, self.3, self.4);
    }

    fn remove(self, class_list: &DomTokenList) {
        let _ = class_list.remove_5(self.0, self.1, self.2, self.3, self.4);
    }
}

/// Switches a class/classes for an element
/// 
/// To be exact it adds one class while removing the other.
pub fn use_swap_class<E, M, C1, C2>(
    target: E,
    remove_class: C1,
    add_class: C2,
) 
where
    E: IntoElementMaybeSignal<Element, M>,
    M: ?Sized,
    C1: ToClassDef,
    C2: ToClassDef,
{
    let target: ElementMaybeSignal<Element>  = target.into_element_maybe_signal();
    if let Some(node) = target.get_untracked() {
        let class_list = node.class_list();
        remove_class.remove(&class_list);
        add_class.add(&class_list);
    }
}

/// Adds class/classes to the class list
pub fn use_add_class<E, M, C>(
    target: E,
    add_class: C,
)
where
    E: IntoElementMaybeSignal<Element, M>,
    M: ?Sized,
    C: ToClassDef,
{
    let target = target.into_element_maybe_signal();
    if let Some(node) = target.get_untracked() {
        console_log(&format!("Adding classes {add_class:#?} to the element"));
        let class_list = node.class_list();
        add_class.add(&class_list);
    }
    else {
        console_log("Adding class skipped, no element in reference");
    }
}

/// Removes class/classes from the class list
pub fn use_remove_class<E, M, C>(
    target: E,
    remove_class: C,
)
where
    E: IntoElementMaybeSignal<Element, M>,
    M: ?Sized,
    C: ToClassDef,
{
    let target = target.into_element_maybe_signal();
    if let Some(node) = target.get_untracked() {
        console_log(&format!("Removing classes {remove_class:#?} to the element"));
        let class_list = node.class_list();
        remove_class.remove(&class_list);
    }
}