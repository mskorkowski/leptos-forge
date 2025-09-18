//! Example of using a store to track the state of the component
//! 

use js_sys::wasm_bindgen::JsValue;
 use leptos::ev::{CustomEvent, MouseEvent};
use leptos::leptos_dom::logging::console_log;
use js_sys::{Array, JsString};
use leptos_node_ref::AnyNodeRef;
use leptos_use::use_document;
use reactive_stores::Store;
use reactive_stores::{Field, Patch};

use leptos::prelude::*;
use utils_leptos::css::use_swap_class;
use utils_leptos::stores::stored_ref::StoredRef;

use forge::Story;

/// Description of the [ComponentWithAStoreStory]
const WIDGET_DESC: &str = r############"
# Component state
# StoredRef

Example shows how to use `StoredRef` to track the node references inside the `Store`.
"############;

/// class for making a button visible as clickable
const BUTTON_CLASS: &str = "
    p-2
    forge-text-standard
    bg-forgeblue-800
    hover:bg-forgeblue-700 
    text-forgeblue-100 
    border-2
    border-forgeblue-800
    justify-center
    items-center
    h-5.5
    inset-y-16/100
    appearance-none
    rounded-sm
    cursor-pointer
    leading-0
    active:bg-forgeblue-500
    active:border-forgeblue-500
    pointer-events-auto
";

/// state of the counter itself
#[derive(Debug, Clone, Store, Patch)]
struct CounterState {
    /// counts the clicks on the button
    counter: usize,
    /// id of the counter
    id: usize,
}

/// Main state of the widget
#[derive(Debug, Clone, Store, Patch)]
struct WidgetState {
    /// counts the clicks on the button
    counter: CounterState,
    /// list of counters
    #[store(key: usize = |counter| counter.id)]
    counters: Vec<CounterState>,
    /// focus
    focus: StoredRef,
}

impl Default for WidgetState {
    fn default() -> Self {
        WidgetState {
            counter: CounterState{ 
                counter: 0,
                id: 1,
            },
            counters: vec![
                CounterState{ 
                    counter: 1,
                    id: 2,
                },
                CounterState{
                    counter: 2,
                    id: 3,
                }
            ],
            focus: StoredRef::Empty,
        }
    }
}

#[component]
fn ComponentWithAStore() -> impl IntoView {
    let store = Store::new(WidgetState::default());

    let on_click = move |_| {
        if let StoredRef::Some(node) = store.focus().get_untracked() {
            let event = CustomEvent::new("fire").unwrap();
            node.dispatch_event(&event).unwrap();
        }
    };

    let on_mousemove = move |event: MouseEvent| {
        let x = event.client_x();
        let y = event.client_y();
        if let Some(document) = &(*use_document()) {

            if let Some(element) = document.element_from_point(
                    x as f32,
                    y as f32,
                ) {

                console_log(&format!("found element {} at the position ({x},{y})", element.tag_name()));
                let class_list = element.class_list();
                console_log(&format!("class list {:?}", class_list.entries().into_iter().map(|entry| {
                    match entry {
                        Ok(value) => {
                            let arr: Array = Array::from(&value);
                            let class_name: JsValue = arr.get(1);
                            let str: JsString = JsString::from(class_name);
                            format!("{str}")
                        }
                        Err(_) => "error".to_string(),
                    }
                }).collect::<Vec<String>>()));

                if element.class_list().contains("mouse-move-target") {
                    console_log("is the mouse-move-target");
                    if let Ok(event) = CustomEvent::new("click") {
                        console_log(&format!("element is {element:?}"));
                        match element.dispatch_event(&event) {
                            Ok(value) => {
                                console_log(&format!("dispatch event value is {value:?}"));
                            }
                            Err(err) => {
                                console_log(&format!("dispatch event error is {err:?}"));
                            }
                        }
                    }
                    else {
                        console_log("cant create an event");
                    }
                }
                else {
                    console_log("not a mouse-move-target");
                }
            }
        }
    };

    view!{
        <div class="">
            <input
                type="text" 
                class=format!("{BUTTON_CLASS} select-none")
                on:click=on_click
                on:mousemove=on_mousemove
                value="Increment focused"
            />
            <hr/>
            <For
                each = move || store.counters()
                key = |counter| counter.id().get()
                let:counter
            >
                <ChildComponentWithAStore
                   store
                   counter
                />
            </For>

        </div>
    }
}

#[component]
fn ChildComponentWithAStore(
    /// store with the state of thw widget
    store: Store<WidgetState>,
    /// counter
    #[prop(into)]
    counter: Field<CounterState>,
) -> impl IntoView {

    let my_ref = AnyNodeRef::new();

    let on_click =  move |_| {
        ChildComponentWithAStoreProps::on_click(my_ref, store);
    }; 

    view!{
        <div class="pointer-events-none">
            <button 
                class=format!("mouse-move-target {BUTTON_CLASS}")
                on:click=on_click
                on:fire=move |_e: CustomEvent | {
                    let c = counter.counter();
                    c.patch(c.get() + 1);
                }
                node_ref=my_ref
                
            >Focus me</button>
            <div class="pointer-events-none">Counter: {counter.counter()}</div>
        </div>
    }
}

impl ChildComponentWithAStoreProps {
    /// Handles the on_click event for the child component
    fn on_click(node_ref: AnyNodeRef, store: Store<WidgetState>) {
        let current = store.focus().get_untracked();
        if let Some(node) = node_ref.get() {
            let new = StoredRef::new(node);
            if new != current {
                use_swap_class(current, "border-red-500", "border-forgeblue-800");
                use_swap_class(node_ref, "border-forgeblue-800", "border-red-500");
            }
            store.focus().patch(new);
        }
    }
}

/// Story for [ComponentWithAStore]
#[derive(Debug, Clone, Copy, Default)]
pub struct ComponentWithAStoreStory {}


impl Story for ComponentWithAStoreStory {
    fn description(&self) -> &'static str {
        WIDGET_DESC
    }   

    fn controls(&self) -> AnyView {
        ().into_any()
    }

    fn view(&self) -> AnyView {
        view!{
            <ComponentWithAStore/>
        }.into_any()
    }
}