//! Side panel shows the description and allows to play tests for the component
//! 

use leptos::prelude::*;
use utils::prelude::ThreadSafe;
use utils_leptos::signal::URwSignal;

/// Returns a name of the tab
pub trait TabName 
where
    Self: Sized
{
    /// Name of the tab
    fn name(&self) -> &'static str;
    /// Html id sane string
    /// 
    /// This value will be combined with an id of the [TabPanel] to generate unique id for the radio button and label.
    fn html_id(&self) -> &'static str;
    /// Creates an instance of selected value based on the string representation
    fn try_from_name(name: &str) -> Option<Self>;
}

/// Defines a tab on the tab list
pub trait Tab<O: PartialEq>: ThreadSafe {
    /// Id of the tab. This is used to identify which tab is selected
    fn id(&self) -> O;
    /// View which should be shown when this tab has been activated
    fn view(&self) -> AnyView;
}

#[component]
pub fn TabPanel<S, O>(
    /// Unique id of the tab panel
    id: S,
    /// List of tabs to be shown
    tabs: Vec<Box<dyn Tab<O>>>,
    /// Selected tab
    selector: URwSignal<O>,
) -> impl IntoView 
where 
    S: ToString,
    O: ThreadSafe + Clone + PartialEq + TabName,
{

    let tab_selector = selector.map(
        |v| {
            v.name().to_string()
        },
        |v, new_name| {
            if let Some(new_value) = O::try_from_name(&new_name) {
                *v=new_value;
            }
        }
    );

    let tabpanel_id = id.to_string();
    let buttons = tabs.
        iter().
        map(|tab| {
            let tabpanel_id = tabpanel_id.clone();
            let id = tab.id();
            view!{
                <TabButton id selector=tab_selector tabpanel_id/>
            }
        }).
        collect_view();

    let views = tabs.
        into_iter().
        map(|tab| {
            view!{
                <TabView tab selector />
            }
        }).
        collect_view();

    view!{
        <div id={tabpanel_id} class="leptos-forge-tab-panel overflow-auto">
            <div class="">
                { buttons }
            </div>
            <div>
                { views }
            </div>
        </div>
    }
}

#[component]
fn TabButton<S, O>(
    /// id of the tab panel element
    tabpanel_id: S,
    /// id of the tab to be shown
    id: O,
    /// Selected tab signal
    selector: URwSignal<String>
) -> impl IntoView 
where
    S: ToString,
    O: ThreadSafe + Clone + TabName 
{
    let tab_button_id = format!("{}-{}", tabpanel_id.to_string(), id.html_id());
    let name = id.name();
    // let on_click = move |_: MouseEvent| selector.set(name.to_string());
    view!{
        <div class="inline-block">
            <input id={tab_button_id.clone()} class="peer hidden" name={tabpanel_id.to_string()} value={name} type="radio" bind:group=selector /> 
            <label for=tab_button_id class="px-2 py-1 text-sm bg-forgegray-300 peer-checked:bg-forgeblue-300">{ name }</label>
        </div>
    }
}

/// Component showing the content of the single tab
#[component]
fn TabView<O>(
    /// Tab to be shown
    tab: Box<dyn Tab<O>>,
    /// Selected tab signal
    #[prop(into)]
    selector: URwSignal<O>,
) -> impl IntoView 
where
    O: ThreadSafe + Clone + PartialEq,
{
    let id = tab.id();
    view!{
        <Show when= move ||{ selector.get() == id }>
            { tab.view() }
        </Show>
    }
}