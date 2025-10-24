//! Single select widget

// use leptos::ev::CustomEvent;
// use leptos::ev::FocusEvent;
// use leptos::ev::KeyboardEvent;
// use leptos::ev::MouseEvent;
// use leptos::web_sys::HtmlInputElement;
// use leptos::leptos_dom::logging::console_log;
// use leptos::prelude::*;
// use leptos_node_ref::AnyNodeRef;
// use leptos_use::use_document;
// use leptos::wasm_bindgen::JsCast;
// use reactive_graph::traits::Get;

// use reactive_stores::Patch;
// use reactive_stores::PatchField;
// use reactive_stores::Store;
// use reactive_stores::Field;

// use floating_ui_leptos::use_floating;
// use floating_ui_leptos::UseFloatingOptions;
// use floating_ui_leptos::UseFloatingReturn;

// use reactive_stores::StorePath;
// use utils::prelude::ThreadSafe;
// use utils_leptos::css::use_add_class;
// use utils_leptos::css::use_remove_class;
// use utils_leptos::css::use_swap_class;
// use utils_leptos::stores::stored_ref::StoredRef;
// use uuid::Uuid;

// use crate::model::Keyed;
// use crate::primitives::input::button::ClearInputButton;
// use crate::primitives::input::TextInput;
// use crate::primitives::label::TextFieldLabel;

// use utils_leptos::signal::URwSignal;

// /// Item that can be selected in the single select widget
// #[derive(Debug, Clone, Store, Patch)]
// struct SingleSelectItem<Value>
// where
//     Value: Clone + PatchField,
// {
//     /// value of the item
//     value: Value,
//     /// reference to the node that represents the item
//     node_ref: StoredRef,
// }

// impl<Value> Keyed for SingleSelectItem<Value>
// where
//     Value: Clone + Keyed + PatchField,
// {
//     fn key(&self) -> &Uuid {
//         self.value.key()
//     }
// }

// #[allow(clippy::to_string_trait_impl)]
// impl<Value> ToString for SingleSelectItem<Value>
// where
//     Value: ToString + Clone + PatchField,
// {
//     fn to_string(&self) -> String {
//         self.value.to_string()
//     }
// }

// /// Selected element
// #[derive(Debug, Clone, Store)]
// struct Selection{
//     /// key of the selected element
//     key: Uuid,
//     /// index in values
//     index: usize,
//     /// node_ref of last selected element
//     node_ref: StoredRef,
// }

// impl PatchField for Selection {
//     fn patch_field(
//             &mut self,
//             new: Self,
//             path: &StorePath,
//             notify: &mut dyn FnMut(&StorePath),
//     ) {
//         console_log("Patching selection");
//         if self.key != new.key {
//             console_log("patch selection key changed");
//             *self = new;
//             notify(path);
//         }
//         else {
//             console_log("patch selection key unchanged")
//         }
//     }
// }

// /// State of the dropdown menu
// #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Store)]
// enum DropdownState {
//     /// Opened by mouse event
//     ClickOpen,
//     /// Opened by other means that mouse event
//     Open,
//     /// Closed
//     #[default]
//     Closed,
//     /// Forces dropdown to be opened
//     ForceOpen,
// }

// impl DropdownState {
//     /// Returns the next state of the popover
//     ///
//     /// | Current state | `click` | Result       |
//     /// |:--------------|:-------:|:-------------|
//     /// | ClickOpen     | any     | Open         |
//     /// | Open          | any     | Closed       |
//     /// | Closed        | true    | ClickOpen    |
//     /// | Closed        | false   | Open         |
//     /// | ForceOpen     | any     | ForceOpen    |
//     fn toggle(self, click: bool) -> Self {
//         match self {
//             DropdownState::ClickOpen => DropdownState::Open,
//             DropdownState::Open => DropdownState::Closed,
//             DropdownState::Closed => if click {
//                 DropdownState::ClickOpen
//             } else {
//                 DropdownState::Open
//             }
//             DropdownState::ForceOpen => DropdownState::ForceOpen,
//         }
//     }

//     /// If this method returns `true` it means that popover
//     /// should be visible
//     fn is_open(&self) -> bool {
//         *self != DropdownState::Closed
//     }
// }

// impl PatchField for DropdownState {
//     fn patch_field(
//         &mut self,
//         new: Self,
//         path: &StorePath,
//         notify: &mut dyn FnMut(&StorePath),
//     ) {
//         if *self != new {
//             *self = new;
//             notify(path);
//         }
//     }
// }

// /// Model for the single sel
// #[derive(Store, Clone, Patch)]
// struct SingleSelectModel<Value>
// where
//     Value: Clone + Keyed + PatchField,
// {
//     /// id of the item with focus
//     selection: Option<Selection>,
//     /// list of items to choose from
//     #[store(key: Uuid = |counter| *counter.value.key())]
//     items: Vec<SingleSelectItem<Value>>,
//     /// how many items are in the list
//     count: usize,
//     /// state of the dropdown menu
//     dropdown: DropdownState,
// }

// /// Select field allowing the selection of the single item from a list of options
// ///
// #[component]
// pub fn SingleSelect<
//     Value: ThreadSafe + Clone + ToString + Keyed + PatchField,
//     S1: ToString,
// >(
//     /// id of the select field
//     id: S1,
//     /// Value of the label for the field
//     #[prop(into)]
//     label: Signal<String>,
//     /// value of the select
//     ///
//     /// if this signal has a `None` value that means no value has been selected
//     #[prop(into)]
//     value: URwSignal<Option<Value>>,
//     /// possible values for the select
//     items: Vec<Value>,
// ) -> impl IntoView {
//     let store: Store<SingleSelectModel<Value>> = Store::new(SingleSelectModel{
//         selection: None,
//         count: items.len(),
//         items: items.into_iter().map(|item| SingleSelectItem{
//             value: item,
//             node_ref: StoredRef::Empty,
//         }).collect(),
//         dropdown: DropdownState::ForceOpen,
//     });

//     let reference_ref = AnyNodeRef::new();
//     let floating_ref = AnyNodeRef::new();

//     let UseFloatingReturn{
//         floating_styles,
//         ..
//     } = use_floating(
//         reference_ref,
//         floating_ref,
//         UseFloatingOptions::default(),
//     );

//     // let selection: URwSignal<Arc<Option<EventTarget>>> = URwSignal::new(Arc::new(None));

//     // let dropdown_state = store.dropdown();

//     let show_dropdown = Signal::derive(move || {
//         match store.dropdown().get() {
//             DropdownState::Closed => false,
//             DropdownState::Open | DropdownState::ClickOpen | DropdownState::ForceOpen => true,
//         }
//     });

//     // let hide_dropdown = move |_event: FocusEvent| {
//     //     match store.dropdown().get_untracked() {
//     //         DropdownState::ForceOpen => {},
//     //         _ => store.dropdown().set(DropdownState::Closed)
//     //     };
//     // };

//     // let toggle_dropdown = move |_event: MouseEvent| {
//     //     match store.dropdown().get() {
//     //         DropdownState::Closed => store.dropdown().set(DropdownState::ClickOpen),
//     //         DropdownState::Open | DropdownState::ClickOpen => store.dropdown().set(DropdownState::Closed),
//     //         DropdownState::ForceOpen => {},
//     //     }
//     // };

//     let focus = move |_event: FocusEvent| {
//         let resolved_state = store.dropdown().get_untracked();

//         if resolved_state == DropdownState::Closed {
//             store.dropdown().set(DropdownState::Open);
//         }
//     };

//     // Firefox triggers the `mouseup` event on the `input` element if we focus the select
//     // and while mouse button is pressed move the mouse over the selection item
//     //
//     // Chromium browsers fire the `mouseup` event on the element above which the mouse is
//     // let mouseup = move |e: MouseEvent| {
//     //     console_log("mouseup - input");
//     //     let resolved_state = store.dropdown().get();
//     //         if let Some(target) = e.target() &&
//     //             let Some(document) = &(*use_document()) &&
//     //             let Some(element) = document.element_from_point(
//     //                 e.client_x() as f32,
//     //                 e.client_y() as f32,
//     //             )
//     //         {
//     //         console_log(&format!("target is {target:?}"));
//     //         if element.class_list().contains("select-dropdown-item") {
//     //             if let Ok(event) = CustomEvent::new("mouseup") {

//     //                 console_log(&format!("element is {element:?}"));
//     //                 match element.dispatch_event(&event) {
//     //                     Ok(value) => {
//     //                         console_log(&format!("dispatch event value is {value:?}"));
//     //                         return;
//     //                     }
//     //                     Err(err) => {
//     //                         console_log(&format!("dispatch event error is {err:?}"));
//     //                     }
//     //                 }
//     //             }
//     //         }

//     //         let target = target.value_of();
//     //         if target.has_type::<Element>() {
//     //             let target = target.unchecked_into::<Element>();
//     //             if target == element { // we've clicked on the input element
//     //                 console_log("target is the same element as the one targeted");
//     //                 match resolved_state {
//     //                     DropdownState::Closed => {},
//     //                     DropdownState::Open => {
//     //                         store.dropdown().set(DropdownState::Closed);
//     //                     },
//     //                     DropdownState::ClickOpen => {
//     //                         store.dropdown().set(DropdownState::Open);
//     //                     },
//     //                     DropdownState::ForceOpen => {}
//     //                 }
//     //             }
//     //         }
//     //         else {
//     //             console_log("target is not an element");
//     //         }
//     //     };
//     // };

//     let selected = value.map(
//         |v| {
//             console_log("value selected");
//             if let Some(value) = v {
//                 console_log(&format!("selected value is {}", value.to_string()));
//                 value.to_string()
//             }
//             else {
//                 console_log("Selected empty value");
//                 String::new()
//             }
//         },
//         |_from, _new| {
//             // let's ignore updates for now
//         }
//     );

//     let clear_button_visibility = Signal::derive(move || {
//         value.get().is_some()
//     });

//     let clear = value.map(
//         |v| {
//             v.is_some()
//         },
//         move |from, new| {
//             console_log("clear button pressed");
//             if new {
//                 let _ = from.take();
//                 if let Some(input) = reference_ref.get() {
//                     if input.has_type::<HtmlInputElement>() {
//                         let input = input.unchecked_into::<HtmlInputElement>();
//                         match input.focus() {
//                             Ok(_) => {
//                                 console_log("input focused");
//                             }
//                             Err(e) => {
//                                 console_log(&format!("input focus error {e:?}"));
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     );

//     let mousemove = |event: MouseEvent| {
//         console_log("mousemove event");
//         if let Some(document) = &(*use_document()) &&
//             let Some(element) = document.element_from_point(
//                 event.client_x() as f32,
//                 event.client_y() as f32,
//             ) &&
//             element.class_list().contains("leptos-forge-select-dropdown-item") &&
//             let Ok(event) = CustomEvent::new("mouseover")
//         {
//             console_log(&format!("element is {element:?}"));
//             match element.dispatch_event(&event) {
//                 Ok(value) => {
//                     console_log(&format!("dispatch event value is {value:?}"));
//                 }
//                 Err(err) => {
//                     console_log(&format!("dispatch event error is {err:?}"));
//                 }
//             }
//         }
//     };

//     let keydown = move |event: KeyboardEvent| {
//         console_log("keydown event");
//         event.stop_propagation();
//         event.prevent_default();
//         event.cancel_bubble();
//         match event.key().as_str() {
//             "Enter" | "Space" | "ArrowLeft" | "ArrowRight" => {
//                 console_log("Confirm selected");
//                 let model = store.get_untracked();
//                 let selection_field = store.selection();
//                 if let Some(selection) = model.selection &&
//                    let Some(item) = model.items.get(selection.index)
//                 {
//                     value.set(Some(item.value.clone()));
//                     selection_field.set(None);
//                     match store.dropdown().get_untracked() {
//                         DropdownState::ForceOpen => {},
//                         _ => {
//                             store.dropdown().set(DropdownState::Closed);
//                         }
//                     }

//                 }
//                 else {
//                     value.set(None);
//                 }

//             }
//             "ArrowUp" => {
//                 console_log("ArrowUp selected");
//                 // let selection_field = store.selection();
//                 store.with(|model| {
//                     if let Some(selected) = &model.selection {
//                         console_log(&format!("selection {selected:?}"));
//                         if selected.index > 0 {
//                             console_log("selection index > 0");
//                             if let Some(item) = model.items.get(selected.index - 1) {
//                                 console_log(&format!("model has value for index {}", selected.index - 1));
//                                 let node_ref = &item.node_ref;
//                                 use_swap_class(node_ref, "bg-forgeblue-200", "bg-forgeblue-300");
//                                 console_log("grab selection");
//                                 let s = store.selection();
//                                 console_log("grab write for selection");

//                                 console_log("replace selection");
//                                 s.patch(Some(Selection{
//                                     index: selected.index - 1,
//                                     node_ref: item.node_ref.clone(),
//                                     key: *item.key()
//                                 }));
//                                 console_log("remove classes");
//                                 // use_remove_class(selected.node_ref.0.into(), ("bg-forgeblue-300", "bg-forgeblue-200"));
//                                 console_log("ArrowUp is done");
//                             }
//                             else {
//                                 console_log("item not found");
//                             }
//                         }
//                         else {
//                             console_log("selection index == 0");
//                         }
//                     }
//                 });
//             }
//             "ArrowDown" => {
//                 console_log("ArrowDown selected");
//                 store.with(|model| {
//                     match &model.selection {
//                         Some(selected) => {
//                             if selected.index + 1 < model.items.len() &&
//                                let Ok(event) = CustomEvent::new("mouseover") {
//                                 let _ = model.items[selected.index + 1].node_ref.dispatch_event(&event);
//                             }
//                             // else we are already at the end of the list
//                         }
//                         None => {
//                             console_log("ArrowDown no selection yet");
//                             if !model.items.is_empty() &&
//                                let Ok(event) = CustomEvent::new("mouseover") {
//                                 let _ = model.items[0].node_ref.dispatch_event(&event);
//                             }
//                         }
//                     }
//                 });

//             }
//             "Escape" => {
//                 let state = store.dropdown().get_untracked();
//                 match state {
//                     DropdownState::ForceOpen => {},
//                     _ => {
//                         store.dropdown().set(DropdownState::Closed);
//                     }
//                 }
//             }
//             _ => {
//                 console_log("Other key");
//             }
//         }
//     };

//     view!{
//         <div class="leptos-forge-field-box relative pt-8">
//             <ClearInputButton
//                 clear={clear.write_only()}
//                 show={clear_button_visibility}
//             />
//             <TextInput
//                 id={id.to_string()}
//                 text=selected
//                 node_ref=reference_ref
//                 on:focus=focus
//                 // on:blur=hide_dropdown
//                 // on:mousedown=toggle_dropdown
//                 // on:mouseup=mouseup
//                 on:mousemove=mousemove
//                 on:keydown=keydown
//             />
//             <TextFieldLabel
//                 for_id={id.to_string()}
//                 text=label
//             />
//             <Show
//                 when=show_dropdown
//             >
//                 <div
//                     node_ref=floating_ref
//                     style:position = move || floating_styles.get().style_position()
//                     style:top = move || floating_styles.get().style_top()
//                     style:left = move || floating_styles.get().style_left()
//                     style:transform = move || floating_styles.get().style_transform()
//                     style:will-change = move || floating_styles.get().style_will_change()

//                     class="leptos-forge-select-dropdown border-2 border-forgeblue-300 border-b-lg w-full p-2 shadow-lg/20"
//                 >
//                     <ul class="leptos-forge-select-dropdown-list list-none">
//                         <ForEnumerate
//                             each= move || store.items()
//                             key = |item| *item.value().get().key()
//                             let(index,item)
//                         >
//                             <SingleSelectItemComponent
//                                item
//                                index
//                                store
//                             />
//                         </ForEnumerate>
//                     </ul>
//                 </div>
//             </Show>
//         </div>
//     }
// }

// /// Item which can be selected in the single select item list
// #[component]
// fn SingleSelectItemComponent<Value>(
//     /// index of the item on the list
//     #[prop(into)]
//     index: Signal<usize>,
//     /// item to be shown
//     #[prop(into)]
//     item: Field<SingleSelectItem<Value>>,
//     /// store with a single select state
//     store: Store<SingleSelectModel<Value>>,
// ) -> impl IntoView
// where
//     Value: Clone + PatchField + Keyed + ThreadSafe + ToString,
// {
//     let node_ref = AnyNodeRef::new();

//     Effect::new(move || {
//         if let Some(element) =  node_ref.get() {
//             item.node_ref().patch(StoredRef::new(element));
//         }
//     });

//     // let onpointerdown = {
//     //     // again we must move the ownership into the Fn()
//     //     let item = item.clone();
//     //     move |_: PointerEvent| {
//     //         console_log("pointerdown");
//     //         value.set(Some(item.value.clone()));
//     //         store.dropdown().set(DropdownState::Closed);
//     //     }
//     // };
//     // let mouseup = {
//     //     // again we must move the ownership into the Fn()
//     //     let item = item.clone();
//     //     move |_: MouseEvent| {
//     //         console_log("mouseup item");
//     //         value.set(Some(item.value.clone()));
//     //         store.dropdown().set(DropdownState::Closed);
//     //     }
//     // };

//     let mouseover = move |_: MouseEvent| {
//         console_log(&format!("item mouseover {}", index.get_untracked()));
//         let selection = store.selection();
//         match selection.get_untracked() {
//             Some(selected) => {
//                 console_log("selection item mouseover");
//                 if *item.value().get().key() != selected.key {
//                     console_log("selection item mouseover - item change");
//                     use_swap_class(node_ref, "bg-forgeblue-300", "bg-forgeblue-200");
//                     use_remove_class(node_ref, ("bg-forgeblue-300", "bg-forgeblue-200"));
//                     selection.patch(Some(Selection{
//                         key: *item.value().get().key(),
//                         index: index.get_untracked(),
//                         node_ref: node_ref.into(),
//                     }));
//                 }
//             }
//             None => {
//                 console_log("none item mouseover");
//                 use_add_class(node_ref, "bg-forgeblue-300");
//                 selection.patch(Some(Selection{
//                     key: *item.value().get().key(),
//                     index: index.get_untracked(),
//                     node_ref: node_ref.into(),
//                 }));
//             }
//         }
//     };

//     view!{
//         <li>
//             <button
//                 node_ref=node_ref
//                 class="leptos-forge-select-dropdown-item border-2 border-fuchsia-500 mt-1 w-full text-left p-2 hover:bg-forgeblue-200 active:bg-forgeblue-300 pointer-events-auto"
//                 // on:pointerdown=onpointerdown
//                 // on:mouseup=mouseup
//                 on:mouseover=mouseover
//             >
//                 <div class="w-full pointer-events-none">
//                     {move || item.get().to_string()}
//                 </div>
//             </button>
//         </li>
//     }
// }
