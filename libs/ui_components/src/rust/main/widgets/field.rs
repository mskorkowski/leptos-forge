//! Input fields

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos::web_sys::Blob;
use utils_leptos::css::use_swap_class;

use crate::model::Password;
use crate::model::Focus;
use crate::primitives::input::BlobFileInput;
use crate::primitives::input::CodeareaInput;
use crate::primitives::input::PasswordInput;
use crate::primitives::input::TextInput;
use crate::primitives::input::TextareaInput;
use crate::primitives::input::button::ClearInputButton;
use crate::primitives::input::button::PasswordButtonStates;
use crate::primitives::input::button::ToggleInputButton;
use crate::primitives::label::InlineFieldLabel;
use crate::primitives::label::Readonly;
use crate::primitives::label::TextFieldLabel;
use crate::primitives::switch::Switch;

use utils_leptos::signal::URwSignal;


/// TextField widget with label
///
/// # Focus behavior
/// 
/// ## Human interaction
/// 
/// If user clicks on the field the `focus` signal will trigger with [`Focus::In`]
/// If user navigates out of the field the `focus` signal will trigger with [`Focus::Out`]
/// 
/// ## Programmatic focus
/// 
/// If you whish to set the focus on the field you should write [`Focus::Grab`].
/// Widget will send the [`Focus::In`] after the focus was acquired.
/// 
/// If you whish to move the focus out of the field you should write [`Focus::Blur`]
/// Widget will send the [`Focus::Out`] after the focus was removed from the widget.
/// 
/// # Label behavior
/// 
/// 1. If field doesn't have any value then label will be displayed as the placeholder
/// 2. If field receives a focus or value then label will move out of the field to the top left corner
/// 3. If field has value then label will be displayed at the top left corner
/// 4. If field value was removed and focus is lost the label will be animated back as the placeholder
#[component]
pub fn TextField<S1: ToString>(
    /// Value of the text field
    #[prop(into)]
    text: URwSignal<String>,
    /// Value of the label
    #[prop(into)]
    label: Signal<String>,
    /// Id of the text field
    id: S1,
    /// Default value of the text field after clearing the field
    #[prop(optional,into, default=URwSignal::new(None).into())]
    default: Signal<Option<String>>,
    /// Signal controls the focus state of the Text field
    #[prop(into, default=URwSignal::new(Focus::Out))]
    focus: URwSignal<Focus>,
    /// errors to show in the ui
    #[prop(into, default=URwSignal::new(Vec::new()).into())]
    errors: Signal<Vec<String>>,
) -> impl IntoView {
    let clear = URwSignal::new(false);
    let input = AnyNodeRef::new();

    Effect::new(move || {
        if clear.get() && !text.get_untracked().is_empty() {
            if let Some(default) = default.get() {
                text.set(default.to_string());
            } else {
                text.set(String::default());
            }
            clear.set(false);
        }
    });

    let clear_button_visibility = Signal::derive(move || {
        let text = text.get();
        if let Some(default) = default.get() {
            default != text
        } else {
            !text.is_empty()
        }
    });

    view! {
        <div class="leptos-forge-field-box relative">
            <div class="relative pt-8">
                <ClearInputButton clear={clear.write_only()} show={clear_button_visibility} />
                <TextInput id={id.to_string()} text focus node_ref=input/>
                <TextFieldLabel for_id={id.to_string()} text=label/>
            </div>
            {
                move || {
                    errors.with(|v| {
                        if !v.is_empty() {
                            use_swap_class(input, ("border-forgeblue-300", "focus:border-forgeblue-500"), ("border-forgered-800", "focus:border-forgered-500"));

                            let errs = v.
                                iter().
                                map(|e| {
                                    view!{
                                        <li>{e.clone()}</li>
                                    }
                                }).collect_view();

                            view!{
                                <ul class="list-disc list-inside ml-4 text-forgered-800 forge-text-standard">
                                    {errs}
                                </ul>
                            }.into_any()
                        }
                        else{
                            use_swap_class(input, ("border-forgered-800", "focus:border-forgered-500"), ("border-forgeblue-300", "focus:border-forgeblue-500"));
                            ().into_any()
                        }
                    })
                }
            }
        </div>
    }
}

/// Textarea widget with label
#[component]
pub fn Textarea(
    /// Value of the textarea
    #[prop(into)]
    text: URwSignal<String>,
    /// Value of the label
    #[prop(into)]
    label: Signal<String>,
    /// Id of the textarea
    id: &'static str,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-field-box relative pt-8">
            <TextareaInput id=id text=text />
            <TextFieldLabel for_id=id text=label/>
        </div>
    }
}

/// Textarea widget with monospace font
#[component]
pub fn Codearea(
    /// Value of the textarea
    #[prop(into)]
    text: URwSignal<String>,
    /// Value of the label
    #[prop(into)]
    label: Signal<String>,
    /// Id of the textarea
    id: &'static str,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-field-box relative pt-8">
            <CodeareaInput id=id text=text />
            <TextFieldLabel for_id=id text=label/>
        </div>
    }
}

/// File input widget
#[component]
pub fn BlobFile(
    /// url to the blob pointing to the selected file
    #[prop(into, optional, default=None.into())]
    file_url: Option<URwSignal<String>>,
    /// signal with blob
    #[prop(into, optional, default=None.into())]
    file: Option<URwSignal<Blob>>,
    /// signal with a file name
    /// 
    /// If signal contains the value `None`, it means no file is currently selected
    /// If signal is `Some` then, there is a file which was selected
    #[prop(into, optional, default=None)]
    file_name: Option<SignalSetter<Option<String>>>,
    /// value of the label
    #[prop(into)]
    label: Signal<String>,
    /// if of the file input
    id: &'static str,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-field-box relative pt-8 isolate">
            <BlobFileInput id=id file_url=file_url file file_name />
            <TextFieldLabel for_id=id text=label force_z_index=-10 />
        </div>
    }
}

/// Password field widget with label
///
/// 1. If field doesn't have any value then label will be displayed as the placeholder
/// 2. If field receives a focus or value then label will move out of the field to the top left corner
/// 3. If field has value then label will be displayed at the top left corner
/// 4. If field value was removed and focus is lost the label will be animated back as the placeholder
#[component]
pub fn PasswordField<S: ToString>(
    /// Value of the text field
    #[prop(into)]
    password: URwSignal<Password>,
    /// Value of the label
    #[prop(into)]
    label: Signal<String>,
    /// Signal toggling visibility state of the password field
    #[prop(into, optional)]
    state: URwSignal<PasswordButtonStates>,
    /// Id of the text field
    id: S,
) -> impl IntoView {
    // let state = URwSignal::new(PasswordButtonStates::default());

    view! {
        <div class="leptos-forge-field-box relative pt-8">
            <ToggleInputButton<PasswordButtonStates> state=state show={true} />
            <PasswordInput id={id.to_string()} password=password state=state/>
            <TextFieldLabel for_id={id.to_string()} text=label/>
        </div>
    }
}

/// Switch field widget with label
///
/// 1. Label will be shown on the left hand side of the field
#[component]
pub fn SwitchField<S: ToString>(
    /// Value of the switch
    ///
    /// If `true` switch will be moved to the left and marked as active (light blue background)
    /// If `false` switch will be moved to the right and marked as inactive (dark grey background)
    #[prop(into)]
    value: URwSignal<bool>,
    /// Value of the label
    #[prop(into)]
    label: Signal<String>,
    /// Id of the text field
    id: S,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-field-box relative pt-8">

            <InlineFieldLabel for_id={id.to_string()} text=label/>
            <Switch
                id={id.to_string()}
                value=value
            />
        </div>
    }
}

/// Readonly field with label
///
/// 1. Label will be shown on the left hand side of the field
#[component]
pub fn ReadonlyField<S: ToString>(
    /// Value of the switch
    ///
    /// If `true` switch will be moved to the left and marked as active (light blue background)
    /// If `false` switch will be moved to the right and marked as inactive (dark grey background)
    #[prop(into)]
    value: Signal<String>,
    /// Value of the label for the field
    #[prop(into)]
    label: Signal<String>,
    /// Id of the text field
    id: S,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-field-box relative pt-8">
            <InlineFieldLabel for_id={id.to_string()} text=label/>
            <Readonly id={id.to_string()} value=value />
        </div>
    }
}
