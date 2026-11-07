//! Basic button stroy

use forge::Story;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::IntoView;
use leptos::prelude::signal;
use leptos::prelude::view;
use reactive_stores::Store;
use ui_components::model::ButtonClick;
use ui_components::model::Kind;
use ui_components::model::Size;
use ui_components::model::Style;
use ui_components::widgets::button::Button;

use crate::State;
use crate::styles;

/// Story for the basic button
/// 
/// This story is designed to be embedded into the [ButtonsSection][super::ButtonsSection]
#[derive(Debug, Clone, Copy)]
pub struct BasicButtonStory {

}

impl Default for BasicButtonStory {
    fn default() -> Self {
        BasicButtonStory {  }
    }
}

impl Story for BasicButtonStory {
    type Data = State;

    /// Returns an empty description since we are embedding this story into the [ButtonsSection][super::ButtonsSection]
    fn description(&self) -> &'static str {
        ""
    }

    fn view(&self, _data: Store<Self::Data>) -> impl IntoView {
        
        let primary_big: Style = Kind::Primary +
            styles::Buttons::BASIC_SCHEMA +
            Size::Big;

        let primary_normal: Style = Kind::Primary +
            styles::Buttons::BASIC_SCHEMA +
            Size::Normal;

        let primary_small: Style = Kind::Primary +
           styles::Buttons::BASIC_SCHEMA +
           Size::Small;

        let secondary_big: Style = Kind::Secondary +
            styles::Buttons::BASIC_SCHEMA +
            Size::Big;

        let secondary_normal: Style = Kind::Secondary +
            styles::Buttons::BASIC_SCHEMA +
            Size::Normal;

        let secondary_small: Style = Kind::Secondary +
            styles::Buttons::BASIC_SCHEMA +
            Size::Small;

        let ghost_big: Style = Kind::Ghost +
            styles::Buttons::BASIC_SCHEMA +
            Size::Big;

        let ghost_normal: Style = Kind::Ghost +
            styles::Buttons::BASIC_SCHEMA +
            Size::Normal;

        let ghost_small: Style = Kind::Ghost +
            styles::Buttons::BASIC_SCHEMA +
            Size::Small;

        let label = "Click me!";

        let (click_out, click_in) = signal(ButtonClick::Released);

        view!{
            <div class="grid grid-cols-4 grid-rows-4 gap-4">
                <div/>
                <div>Big</div>
                <div>Normal</div>
                <div>Small</div>
                <div>Primary</div>
                <div>
                    <Button 
                        click=click_in
                        style=primary_big
                    >
                        {label}
                    </Button>
                </div>
                <div>
                    <Button 
                        click=click_in
                        style=primary_normal
                    >
                        {label}
                    </Button>
                </div>
                <div>
                    <Button 
                        click=click_in
                        style=primary_small
                    >
                        {label}
                    </Button>
                </div>
                <div>Secondary</div>
                <div>
                    <Button 
                        click=click_in
                        style=secondary_big
                    >
                        {label}
                    </Button>
                </div>
                <div>
                    <Button 
                        click=click_in
                        style=secondary_normal
                    >
                        {label}
                    </Button>
                </div>
                <div>
                    <Button 
                        click=click_in
                        style=secondary_small
                    >
                        {label}
                    </Button>
                </div>
                <div>
                    Ghost
                </div>
                <div>
                    <Button 
                        click=click_in
                        style=ghost_big
                    >
                        {label}
                    </Button>
                </div>
                <div>
                    <Button 
                        click=click_in
                        style=ghost_normal
                    >
                        {label}
                    </Button>
                </div>
                <div>
                    <Button 
                        click=click_in
                        style=ghost_small
                    >
                        {label}
                    </Button>
                </div>
            </div>
        }
    }
}