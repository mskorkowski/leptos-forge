//! Label primitive

use leptos::prelude::*;

/// Label component for inputs like
///
/// - `<input type="text">`
/// - `<textarea>`
/// - `<select>`
/// - ...
///
#[component]
pub fn TextFieldLabel<S: ToString>(
    /// Id of the labeled element
    for_id: S,
    /// Text to be displayed on the label
    text: Signal<String>,
    ///forces custom z-index to the label
    ///
    /// By default label is shown with z-index = 10
    #[prop(optional, into)]
    force_z_index: Option<i32>,
    /// Value of the `test-data` attribute of the label, to be used in testing
    #[prop(optional, into)]
    data_testid: Option<&'static str>,
) -> impl IntoView {
    let z_index = force_z_index.unwrap_or(10);

    let test_data = view! {
        <{..} data-testid=data_testid />
    };

    view! {
        <label
            for={for_id.to_string()} class="leptos-forge-primitives-label absolute forge-text-small text-forgegray-700 duration-300 top-3.5 left-2.4 select-none transform origin-[0] start-2.5 peer-focus:translate-y-0 peer-focus:text-forgegray-700 peer-focus:forge-text-small peer-placeholder-shown:translate-y-12/10 peer-placeholder-shown:cursor-text peer-placeholder-shown:text-forgegray-700 peer-placeholder-shown:forge-text-standard"
            style:z-index={{z_index.to_string()}}
            {..test_data}
        >
            {text}
        </label>
    }
}

/// Label component for inputs other kinds like
///
/// - `<input type="radio">`
/// - `<checkbox>`
/// - toggle
/// - ...
///
#[component]
pub fn InlineFieldLabel<S: ToString>(
    /// Id of the labeled element
    for_id: S,
    /// Text to be displayed on the label
    text: Signal<String>,
) -> impl IntoView {
    view! {
        <label for={for_id.to_string()} class="leptos-forge-primitives-label forge-text-standard left-2.4 select-none start-2.5 text-forgegray-700">
            {text}
        </label>
    }
}

/// Readonly component which can be used to replace input fields
#[component]
pub fn Readonly<S: ToString>(
    /// id of the element
    id: S,
    /// value to be shown
    value: Signal<String>,
) -> impl IntoView {
    view! {
        <div id={id.to_string()}>
            {value}
        </div>
    }
}
