//! Implementation of the SingleSelect things around the ColorInformation

use leptos::prelude::*;

use crate::widgets::single_select::SingleSelectItemView;

use super::ColorCode;

impl SingleSelectItemView for ColorCode {
    fn selection_list_view(self) -> impl leptos::IntoView {
        use ColorCode::*;
        let base_css = "w-5.5 h-5.5 inline-block justify-center rounded-sm absolute";
        let label_cls = "ml-6 inline-block";
        match self {
            CssClass { cls, name, .. } => {
                let cls = format!("{base_css} {cls}");

                view!{
                    <div class={cls} />
                    <div class={label_cls}>{name}</div>
                }.into_any()
            }
            Color(color) => {
                let name = color.clone();

                view!{
                    <div class={base_css} style:background-color=color />
                    <div class={label_cls}>{name}</div>
                }.into_any()
            }
            Skip => {
                //TODO: Think more about this stuff
                view!{
                    ~~~[SKIP]~~~
                }.into_any()
            }

        }
    }
}