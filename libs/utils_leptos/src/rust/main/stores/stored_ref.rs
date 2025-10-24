//! Provides implementation of the [NodeRef][leptos::prelude::NodeRef] like value which can be used inside the Store

use leptos::leptos_dom::logging::console_error;
use leptos::prelude::GetUntracked;
use leptos::prelude::LocalStorage;
use leptos::prelude::StoredValue;
use leptos::tachys::renderer::dom::Element;
use leptos_node_ref::AnyNodeRef;
use leptos_use::core::ElementMaybeSignalType;
use leptos_use::core::IntoElementMaybeSignalType;
use reactive_stores::PatchField;
use send_wrapper::SendWrapper;
use web_sys::Event;
use web_sys::wasm_bindgen::JsValue;

///  NodeRef like value which can be used inside the Store
///
/// Differences between this and the `NodeRef` (or [AnyNodeRef][leptos_node_ref::AnyNodeRef])
///
/// - that this type is not reactive by itself and for reactive part it depends on the
///   [`Store`][reactive_stores::Store].
/// - this value can be set to be empty ([`StoredRef::None`]) which is impossible in the case of the
///   [`NodeRef`][leptos::prelude::NodeRef] or [`AnyNodeRef`][leptos_node_ref::AnyNodeRef].
/// - this value doesn't implement [`NodeRefContainer`][leptos::tachys::html::node_ref::NodeRefContainer]
///   trait because it's not a `copyable` and `NodeRefContainer` api requires that by using `self` instead
///   of `&self` as the first argument.
#[derive(Debug, Clone, Default)]
pub enum StoredRef {
    /// There is no reference held
    #[default]
    Empty,
    /// There is a reference to the DOM node
    Some(SendWrapper<Element>),
}

impl StoredRef {
    /// Removes the node reference
    pub fn clear(&mut self) -> &mut Self {
        *self = StoredRef::Empty;
        self
    }

    /// Sets the node reference
    pub fn load(&mut self, node: Element) -> &mut Self {
        *self = StoredRef::Some(SendWrapper::new(node));
        self
    }

    /// Create new instance of the [Node] initialized with a DOM node
    pub fn new(node: Element) -> Self {
        StoredRef::Some(SendWrapper::new(node))
    }

    /// Dispatches an event on the node reference
    ///
    /// If the reference points to the nonexisting node then this method will return `Ok(true)` since
    /// no handler was called, so no handler has called `Event::prevent_default()` on the cancelable
    /// event.
    ///
    /// # Errors
    ///
    /// From [MDN](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent):
    ///
    /// > InvalidStateError DomException
    /// >
    /// > Thrown if the event's type was not specified during event initialization.
    ///
    pub fn dispatch_event(&self, event: &Event) -> Result<bool, JsValue> {
        if let StoredRef::Some(element) = self {
            element.dispatch_event(event)
        } else {
            Ok(true)
        }
    }
}

impl PatchField for StoredRef {
    fn patch_field(
        &mut self,
        new: Self,
        path: &reactive_stores::StorePath,
        notify: &mut dyn FnMut(&reactive_stores::StorePath),
    ) {
        if new != *self {
            *self = new;
            notify(path);
        }
    }
}

/// Equality for [Node] is defined as `strict` equality (both [Nodes][Node] must point to the
/// exactly same instance of the html node).
///
/// Internally we are using the [`is_same_node`][https://developer.mozilla.org/en-US/docs/Web/API/Node/isSameNode].
impl PartialEq for StoredRef {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StoredRef::Empty, StoredRef::Empty) => true,
            (StoredRef::Some(a), StoredRef::Some(b)) => a.is_same_node(Some(b)),
            _ => false,
        }
    }
}

impl IntoElementMaybeSignalType<Element, LocalStorage> for StoredRef {
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<Element> {
        let inner = match self {
            StoredRef::Empty => None,
            StoredRef::Some(node) => {
                if node.valid() {
                    Some(node.take())
                } else {
                    console_error(
                        "Accessing the StoredRef from a different thread then it was created. That is not allowed.",
                    );
                    None
                }
            }
        };

        ElementMaybeSignalType::Static(StoredValue::new_local(inner))
    }
}

impl IntoElementMaybeSignalType<Element, LocalStorage> for &StoredRef {
    fn into_element_maybe_signal_type(self) -> ElementMaybeSignalType<Element> {
        let inner = match self {
            StoredRef::Empty => None,
            StoredRef::Some(node) => {
                if node.valid() {
                    Some(node.clone().take())
                } else {
                    console_error(
                        "Accessing the StoredRef from a different thread then it was created. That is not allowed.",
                    );
                    None
                }
            }
        };

        ElementMaybeSignalType::Static(StoredValue::new_local(inner))
    }
}

impl From<AnyNodeRef> for StoredRef {
    fn from(value: AnyNodeRef) -> Self {
        match value.get_untracked() {
            Some(el) => StoredRef::new(el),
            None => StoredRef::Empty,
        }
    }
}

impl From<&AnyNodeRef> for StoredRef {
    fn from(value: &AnyNodeRef) -> Self {
        match value.get_untracked() {
            Some(el) => StoredRef::new(el),
            None => StoredRef::Empty,
        }
    }
}
