//! Primitives containing a input field primitives
pub mod button;

use leptos::attr::Attribute;
use leptos::ev::Event;
use leptos::ev::Targeted;
use leptos::html;

use leptos::prelude::*;
use leptos::web_sys::Blob;
use leptos::web_sys::File;
use leptos::web_sys::HtmlInputElement;
use leptos::web_sys::Url;

use leptos_node_ref::AnyNodeRef;

use utils_leptos::css::use_swap_class;
use utils_leptos::signal::URwSignal;

use crate::model::Password;
use button::ClearInputButton;
use button::PasswordButtonStates;

/// Spread component which applies the class attribute to the element with classes specific for the input field element
pub fn input_class() -> impl Attribute {
    view!{
        <{..} class="leptos-forge-input block w-full forge-text-standard py-1 px-2 peer border-2 border-solid border-forgeblue-800 rounded-sm focus:border-2 focus:border-forgeblue-500 focus:outline-none" />
    }
}

/// Spread component which applies the class attribute to the element with classes specific for the textarea element
pub fn textarea_class() -> impl Attribute {
    view!{
        <{..} class="leptos-forge-textarea block peer py-1 px-2.5 w-full h-full min-h-80 forge-text-standard text-forgegray-900 bg-forgegray-50 rounded-sm border border-forgegray-300 focus:ring-forgeblue-500 focus:border-forgeblue-500" />
    }
}

/// Just a text `<input type=text>`
#[component]
pub fn TextInput<
    IdValue
>(
    /// the value of the input
    #[prop(into)]
    text: URwSignal<String>,
    /// id of the field
    id: IdValue,
    /// node reference to the input element
    #[prop(into, default=AnyNodeRef::new())]
    node_ref: AnyNodeRef,
) -> impl IntoView 
where
    IdValue: ToString   
{
    let id = id.to_string();
    view! {
        <input 
            node_ref=node_ref
            type="text" 
            placeholder=" " 
            {..input_class()} 
            id=id 
            on:input:target=move |ev|{ text.set(ev.target().value()); } 
            prop:value=text
            draggable=false
        />
    }
}

/// Raw textarea with a bit of reactivity
#[component]
pub fn TextareaInput<IdValue>(
    /// the value of the textarea
    text: URwSignal<String>,
    /// id of the field
    id: IdValue,
) -> impl IntoView
where
    IdValue: ToString
{
    let id = id.to_string();
    view! {
        <textarea placeholder=" " {..textarea_class()} id=id on:input:target=move |ev| text.set(ev.target().value()) prop:value=text>
            {text}
        </textarea>
    }
}

/// Spread component which applies the class attribute to the element with classes specific for the textarea element
pub fn codearea_class() -> impl Attribute {
    view!{
        <{..} class="leptos-forge-textarea peer block py-1 px-2.5 w-full h-full min-h-80 forge-text-standard text-forgegray-900 bg-forgegray-50 rounded-sm border border-forgegray-300 focus:ring-forgeblue-500 focus:border-forgeblue-500 font-mono" />
    }
}

/// Raw textarea with monospace font and a bit of reactivity
#[component]
pub fn CodeareaInput<IdValue>(
    /// the value of the textarea
    text: URwSignal<String>,
    /// id of the field
    id: IdValue,
) -> impl IntoView
where
    IdValue: ToString
{
    let id = id.to_string();
    view! {
        <textarea placeholder=" " {..codearea_class()} id=id on:input:target=move |ev| text.set(ev.target().value()) prop:value=text>
            {text}
        </textarea>
    }
}

/// File selection widget returning the url to the selected blo `<input type=file>`
#[component]
pub fn BlobFileInput<IdValue>(
    /// the value of the input
    file: URwSignal<String>,
    /// id of the field
    id: IdValue,
) -> impl IntoView 
where
    IdValue: ToString
{
    let id = id.to_string();
    let file_id = format!("{id}-file");
    let label_id = file_id.clone();
    let file_name_input: NodeRef<html::Input> = NodeRef::new();


    let text = URwSignal::new(String::default());
    let clear = URwSignal::new(false); // create a clear button signal
    let focus = URwSignal::new(false);
    let show_clear_button = URwSignal::new(false);

    let label_clicked = move |_| {
        focus.set(true);

        use_swap_class(
            file_name_input,
            "border-forgeblue-800",
            "border-forgeblue-500",
        );
    };

    let on_input = move |ev: Targeted<Event, HtmlInputElement>| {
        let files = ev.target().files().unwrap();
        for f in 0..files.length() {
            let input_file: File = files.get(f).unwrap();
            let blob: &Blob = &input_file;
            let url: String = Url::create_object_url_with_blob(blob).unwrap();
            focus.set(false);
            file.set(url);
            text.set(input_file.name());
        }

        use_swap_class(
            file_name_input,
            "border-forgeblue-500",
            "border-forgeblue-800",
        );
    };

    let on_cancel = move |_: Event| {
        use_swap_class(
            file_name_input,
            "border-forgeblue-500",
            "border-forgeblue-800",
        );
    };

    Effect::new(move || { // clear the file input
        if clear.get() {
            file.set(String::default());
            text.set(String::default());
        }
    });

    Effect::new(move || {
        if text.get().is_empty() {
            show_clear_button.set(false);
        }
        else {
            show_clear_button.set(true);
        }
    });
    
    view! {
        <label 
            for={label_id} 
            class="leptos-forge-file-blob-input-label block w-full forge-text-standard py-1 px-2 rounded-sm border-2 border-transparent cursor-pointer"
            on:click=label_clicked
        >
            <span inner_html=||{"&nbsp;"} />
            <input 
                type="file" 
                class="hidden" 
                id=file_id 
                on:input:target=on_input 
                on:cancel=on_cancel
                prop:value=file
                aria-hidden
            />
            <ClearInputButton
                clear={clear.write_only()}
                show={show_clear_button.read_only()}
            />
        </label>
        <input 
            type="text" 
            class="leptos-forge-input block w-full forge-text-standard py-1 px-2 rounded-sm peer absolute -z-10 top-8 forge-text-standard pl-2.5 border-2 border-solid border-forgeblue-800 outline-none"
            placeholder=" "
            node_ref=file_name_input
            id=id on:input:target=move |ev| text.set(ev.target().value()) 
            prop:value=text 
            disabled
        />
    }   
}



/// Just a text `<input type=password>`
#[component]
pub fn PasswordInput<IdValue>(
    /// the value of the input
    #[prop(into)]
    password: URwSignal<Password>,
    /// state toggling between the `*` and plaintext in password field
    #[prop(into)]
    state: Signal<PasswordButtonStates>,
    /// id of the field
    id: IdValue,
) -> impl IntoView 
where
    IdValue: ToString
{
    let id = id.to_string();

    let on_input = move |ev: Targeted<Event, HtmlInputElement>| {
        let value = ev.target().value();
        password.set(Password::new(value));
    };

    let password_value = move || {
        // Safety: This is "safe" as it can get since browser must be able to send a password value to the server
        // using a form
        #[allow(unsafe_code)]
        unsafe {
            password.get().get_raw_password_value().to_string()
        }
    };

    let field_type = Signal::derive(move || {
        use PasswordButtonStates::*;
        match state.get() {
            Hidden => "password",
            Visible => "text"
        }
    });

    view! {
        <input 
            type=field_type 
            placeholder=" " 
            {..input_class()} 
            id=id 
            on:input:target=on_input 
            prop:value=password_value
        />
    }
}