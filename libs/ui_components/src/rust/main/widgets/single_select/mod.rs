//! Single select widget

mod float;
mod keyboard;
mod model;
mod selection;

use leptos::ev::CustomEvent;
use leptos::ev::FocusEvent;
use leptos::ev::KeyboardEvent;
use leptos::ev::MouseEvent;
use leptos::ev::PointerEvent;
use leptos::leptos_dom::logging::console_error;
use leptos::tachys::renderer::dom::Element;
use leptos::web_sys::HtmlInputElement;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_use::use_document;
use leptos::wasm_bindgen::JsCast;
use reactive_graph::traits::Get;

use reactive_stores::Patch;
use reactive_stores::PatchField;
use reactive_stores::Store;
use reactive_stores::Field;

use floating_ui_leptos::use_floating;
use floating_ui_leptos::UseFloatingOptions;
use floating_ui_leptos::UseFloatingReturn;

use utils::prelude::ThreadSafe;
use utils_leptos::css::use_add_class;
use utils_leptos::css::use_remove_class;
use utils_leptos::css::use_swap_class;
use utils_leptos::stores::stored_ref::StoredRef;

use crate::model::Keyed;
use crate::primitives::input::button::ClearInputButton;
use crate::widgets::single_select::float::FloatingController;
use crate::widgets::single_select::keyboard::KeyboardController;
pub use crate::widgets::single_select::model::DropdownState;
use crate::widgets::single_select::model::Selection;
use crate::widgets::single_select::model::SingleSelectItem;
use crate::widgets::single_select::model::SingleSelectItemStoreFields;
use crate::widgets::single_select::model::SingleSelectModel;
use crate::widgets::single_select::model::SingleSelectModelStoreFields;
use crate::widgets::single_select::selection::SelectionController;

use utils_leptos::signal::URwSignal;

/// This trait groups all required traits which value type must implement by 
/// values for [SingleSelect] to work.
/// 
/// - [ThreadSafe]
/// - Clone 
/// - ToString 
/// - [Keyed] 
/// - [PatchField] - if possible you should derive [Patch] to implement
pub trait SingleSelectValue: ThreadSafe + Clone + Keyed + PatchField + SingleSelectItemView {}


impl<V: ThreadSafe + Clone + Keyed + PatchField + SingleSelectItemView> SingleSelectValue for V {}

/// Trait which needs to be implemented by the `Value` type so it can be displayed 
/// using the [SingleSelect]
pub trait SingleSelectItemView: Sized{
    /// Method is called for every item, so it can display itself on the selection
    /// list
    fn selection_list_view(self) -> impl IntoView;

    /// Method is called to show the selected item
    /// 
    /// ## Default implementation
    /// 
    /// By default it returns the same view as [`selection_list_view`][SingleSelectItemView::selection_list_view]
    fn selected_item_view(self) -> impl IntoView {
        self.selection_list_view()
    }
}

/// Select field allowing the selection of the single item from a list of options
///
/// # Type arguments
/// 
/// - **Value** - Type of an element in the single select view
/// - **S1** - Type of unique id attribute of the single select
/// - **ChildViewFn** - Function taking a an instance of the Value and returning the view for the child
/// - **ChildView** - Type of the concrete view
#[component]
pub fn SingleSelect<
    Value,
    S1,
>(
    /// id of the select field
    id: S1,
    /// Value of the label for the field
    #[prop(into)]
    label: Signal<String>,
    /// value of the select
    ///
    /// if this signal has a `None` value that means no value has been selected
    #[prop(into)]
    value: URwSignal<Option<Value>>,
    /// possible values for the select
    #[prop(into)]
    items: Signal<Vec<Value>>,
    /// Initial state of the selection menu
    #[prop(default=DropdownState::Closed)]
    initial_state: DropdownState,
) -> impl IntoView 
where
    Value: SingleSelectValue,
    S1: ToString,
{
    let id=id.to_string();
    let (dropdown_id, _) = signal(format!("{id}-dropdown"));

    let store: Store<SingleSelectModel<Value>> = Store::new(SingleSelectModel{
        selection: None,
        count: items.get_untracked().len(),
        items: items.get_untracked().into_iter().enumerate().map(|(idx, item)| SingleSelectItem{
            value: item,
            node_ref: StoredRef::Empty,
            id: format!("{id}-item-{idx}"),
        }).collect(),
        dropdown: initial_state,
    });

    let reference_ref = AnyNodeRef::new();
    let floating_ref = AnyNodeRef::new();

    let UseFloatingReturn{
        floating_styles,
        ..
    } = use_floating(
        reference_ref,
        floating_ref,
        UseFloatingOptions::default(),
    );



    // let selection: URwSignal<Arc<Option<EventTarget>>> = URwSignal::new(Arc::new(None));

    // let dropdown_state = store.dropdown();

    let show_dropdown = Signal::derive(move || {
        store.dropdown().get().is_open()
    });

    let toggle_dropdown = move |_event: MouseEvent| {
        match store.dropdown().get_untracked() {
            DropdownState::Closed => {
                FloatingController.show(store, true);
            }
            DropdownState::Open | DropdownState::ClickOpen => FloatingController.hide(store),
            DropdownState::ForceOpen => {},
        }
    };

    let span_toggle_dropdown = move |event: MouseEvent| {
        toggle_dropdown(event)
    };

    let focus = move |_event: FocusEvent| {
        FloatingController.show(store, false);
    };

    let blur = move | _event: FocusEvent| {
        FloatingController.hide(store);
    };

    // Firefox triggers the `mouseup` event on the `input` element if we focus the select
    // and while mouse button is pressed move the mouse over the selection item
    //
    // Chromium browsers fire the `mouseup` event on the element above which the mouse is
    let mouseup = move |e: MouseEvent| {
        let resolved_state = store.dropdown().get();
            if let Some(target) = e.target() &&
                let Some(document) = &(*use_document()) &&
                let Some(element) = document.element_from_point(
                    e.client_x() as f32,
                    e.client_y() as f32,
                )
            {
            if element.class_list().contains("leptos-forge-select-dropdown-item") &&
                let Ok(event) = CustomEvent::new("mouseup") && 
                let Err(err) = element.dispatch_event(&event) {
                    console_error(&format!("dispatch event error is {err:?}"));
                }
            

            let target = target.value_of();
            if target.has_type::<Element>() {
                let target = target.unchecked_into::<Element>();
                if target == element { // we've clicked on the input element
                    match resolved_state {
                        DropdownState::Closed => {},
                        DropdownState::Open => {
                            store.dropdown().set(DropdownState::Closed);
                        },
                        DropdownState::ClickOpen => {
                            store.dropdown().set(DropdownState::Open);
                        },
                        DropdownState::ForceOpen => {}
                    }
                }
            }
            // else {
            //     console_log("target is not an element");
            // }
        };
    };

    let clear = value.map(
        |v| {
            v.is_some()
        },
        move |from, new| {
            if new {
                let _ = from.take();
                if let Some(input) = reference_ref.get() &&
                   input.has_type::<HtmlInputElement>() {
                 
                    let input = input.unchecked_into::<HtmlInputElement>();

                    if let Err(e) = input.focus() {
                        console_error(&format!("input focus error {e:?}"));
                    }
                }
            }
        }
    );

    let mousemove = |event: MouseEvent| {
        if let Some(document) = &(*use_document()) &&
            let Some(element) = document.element_from_point(
                event.client_x() as f32,
                event.client_y() as f32,
            ) &&
            element.class_list().contains("leptos-forge-select-dropdown-item") &&
            let Ok(event) = CustomEvent::new("mouseover") && 
            let Err(err) = element.dispatch_event(&event) {
                console_error(&format!("dispatch event error is {err:?}"));
            }
    };

    let keydown = move |event: KeyboardEvent| {
        event.stop_propagation();
        event.prevent_default();
        event.cancel_bubble();
        match event.key().as_str() {
            "Enter" | "Space" | "ArrowLeft" | "ArrowRight" => {
                KeyboardController.enter(store, value);
            }
            "ArrowUp" => {
                KeyboardController.arrow_up(store, value.into());
            }
            "ArrowDown" => {
                KeyboardController.arrow_down(store, value.into());
            }
            "Escape" => {
                KeyboardController.escape(store);
            }
            _ => {}
        }
    };

    let visible = move || value.get().is_some();
    let label_css = move || if visible() {
        "leptos-forge-primitives-label absolute forge-text-small text-forgegray-700 duration-300 top-3.5 left-2.4 select-none transform origin-[0] start-2.5 peer-focus:translate-y-0 peer-focus:text-forgegray-700 peer-focus:forge-text-small"
    }
    else {
        "leptos-forge-primitives-label absolute text-forgegray-700 duration-300 top-3.5 left-2.4 select-none transform origin-[0] start-2.5 min-h-[20px] peer-focus:translate-y-0 peer-focus:text-forgegray-700 peer-focus:forge-text-small translate-y-12/10 cursor-text forge-text-standard"
    };

    let input_css = move || {
        "leptos-forge-input block w-full min-h-[2rem] select-none forge-text-standard py-1 px-2 peer border-2 border-solid border-forgeblue-300 rounded-sm focus:border-2 focus:border-forgeblue-500 focus:outline-none"
    };

    let item_view = move || { 
        match value.get() {
            Some(v) => v.selected_item_view().into_any(),
            None => ().into_any()
        }
    };

    view!{
        <div class="leptos-forge-field-box relative pt-8">
            <ClearInputButton
                clear={clear.write_only()}
                show={visible}
            />
            <div
                id
                node_ref=reference_ref
                on:focus=focus
                on:blur=blur
                on:mousedown=toggle_dropdown
                on:mouseup=mouseup
                on:mousemove=mousemove
                on:keydown=keydown
                class=input_css
                tabindex=0
                role="combobox"
                aria-label={label}
                aria-expanded=show_dropdown
                aria-controls=dropdown_id
                aria-activedescendant=move || store.selection().get().map(|selection| {
                    selection.id
                })
                aria-haspopup="listbox"
            >
                <Show when=visible>
                    { item_view }
                </Show>
            </div>
            <span
                class=label_css
                on:mousedown=span_toggle_dropdown
            >
                { label }
            </span>
            <Show
                when={move || show_dropdown.get()}
                
            >
                <div
                    node_ref=floating_ref
                    id=dropdown_id
                    style:position = move || floating_styles.get().style_position()
                    style:top = move || floating_styles.get().style_top()
                    style:left = move || floating_styles.get().style_left()
                    style:transform = move || floating_styles.get().style_transform()
                    style:will-change = move || floating_styles.get().style_will_change()
                    class="leptos-forge-select-dropdown border-2 border-forgeblue-300 border-b-lg w-full p-2 shadow-lg/20 bg-forgeblue-50 z-popup max-h-[300px] h-fit scrollable"
                >
                    <ul class="leptos-forge-select-dropdown-list list-none">
                        <ForEnumerate
                            each= move || store.items()
                            key = |item| item.value().get().key()
                            let(index,item)
                        >
                            <SingleSelectItemComponent
                               item
                               index
                               store
                               value
                            />
                        </ForEnumerate>
                    </ul>
                </div>
            </Show>
        </div>
    }
}

/// Item which can be selected in the single select item list
/// 
/// # Type arguments
/// 
/// - **Value** - Type of element in the single select
///
#[component]
fn SingleSelectItemComponent<Value>(
    /// index of the item on the list
    #[prop(into)]
    index: Signal<usize>,
    /// item to be shown
    #[prop(into)]
    item: Field<SingleSelectItem<Value>>,
    /// store with a single select state
    store: Store<SingleSelectModel<Value>>,
    /// Value of the single select
    #[prop(into)]
    value: URwSignal<Option<Value>>,
) -> impl IntoView
where
    Value: SingleSelectValue,
{
    let node_ref = AnyNodeRef::new();

    Effect::new(move || {
        if let Some(element) =  node_ref.get() {
            item.node_ref().patch(StoredRef::new(element));
        }
    });

    let item_key = item.get_untracked().key();

    let css_classes = if 
        // let Some(selection) = store.selection().get_untracked() &&
        let Some(value) = value.get_untracked() &&
        // (
            // selection.key == item_key ||
             value.key() == item_key 
        // )

    {
        "leptos-forge-select-dropdown-item mt-1 w-full text-left p-2 hover:bg-forgeblue-200 active:bg-forgeblue-300 pointer-events-auto bg-forgeblue-300"
    }
    else {
        "leptos-forge-select-dropdown-item mt-1 w-full text-left p-2 hover:bg-forgeblue-200 active:bg-forgeblue-300 pointer-events-auto"
    };

    let onpointerdown = {
        move |_: PointerEvent| {
            value.set(Some(item.value().get_untracked().clone()));
            FloatingController.hide(store);
        }
    };
    let mouseup = {
        // again we must move the ownership into the Fn()
        move |_: MouseEvent| {
            value.set(Some(item.value().get_untracked().clone()));
            store.dropdown().set(DropdownState::Closed);
        }
    };   

    let mouseover = move |_: MouseEvent| {
        let selection = store.selection();
        match selection.get_untracked() {
            Some(selected) => {
                console_log("Mouseover selected");
                let value = value.get_untracked();
                let item_key = item.value().get_untracked().key();
                if item_key != selected.key {
                    console_log("Mouseover selected - key change");
                    SelectionController.mark_selected(&item.node_ref().get_untracked());
                    if let Some(value) = value &&
                        value.key() == item_key
                    {
                        use_swap_class(node_ref, "bg-forgeblue-200", "bg-forgeblue-300");
                    }
                    else {
                        use_remove_class(node_ref, ("bg-forgeblue-300", "bg-forgeblue-200"));
                    }
                    let item = item.get();
                    selection.patch(Some(Selection{
                        key: item.value.key(),
                        index: index.get_untracked(),
                        node_ref: node_ref.into(),
                        id: item.id.clone(),
                    }));
                }
                else {
                    console_log("Mouseover selected - key unchange");
                }
            }
            None => {
                console_log("Mouseover unselected");
                use_add_class(node_ref, "bg-forgeblue-300");
                let item = item.get();
                selection.patch(Some(Selection{
                    key: item.key(),
                    index: index.get_untracked(),
                    node_ref: node_ref.into(),
                    id: item.id.clone()
                }));
            }
        }
    };

    let item_view = move || {
        let selection_item = item.get();
        let value = selection_item.value;
        value.selection_list_view()
    };

    view!{
        <li>
            <button
                node_ref=node_ref
                class={css_classes}
                on:pointerdown=onpointerdown
                on:mouseup=mouseup
                on:mouseover=mouseover
            >
                <div class="w-full pointer-events-none">
                    { item_view }
                </div>
            </button>
        </li>
    }
}


