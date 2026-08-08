
//! Card ui
//! 
//! You might give it a title or footer
//! 

use leptos::prelude::*;

/// Header of the card
#[slot]
pub struct Header {
    /// content of the header
    children: ChildrenFn
}

/// Footer of the card
#[slot]
pub struct Footer {
    /// content of the footer
    children: ChildrenFn
}

/// Content of the card
#[slot]
pub struct Content {
    /// content of the Content slot
    children: ChildrenFn
}

/// Card to be shown in the ui
/// 
/// Children of the card are the body
#[component]
pub fn Card<
    Id: ToString
>(
    /// Id of the component
    id: Id,
    /// header of the card
    #[prop(optional)]
    header: Option<Header>,
    /// footer of the card
    #[prop(optional)]
    footer: Option<Footer>,
    /// content of the card
    content: Content
) -> impl IntoView {
    let id = id.to_string();
    let content_id = format!("{id}-content");
    let card_classes= "first:mt-0 mt-5 border-2 border-forgeblue-300 p-3 bg-forgegray-50 rounded-md";
    let header_classes= "forge-text-big";
    match (header, footer) {
        (None, None) => {
            view!{
                <div id class=card_classes>
                    <div id=content_id>
                        {(content.children)()}
                    </div>
                </div>
            }.into_any()
        },
        (None, Some(footer)) => {
            let footer_id = format!("{id}-footer");
            view!{
                <div id class=card_classes>
                    <div id=content_id>
                        {(content.children)()}
                    </div>
                    <div id=footer_id>
                        {(footer.children)()}
                    </div>
                </div>
            }.into_any()
        },
        (Some(header), None) => {
            let header_id = format!("{id}-header");
            view!{
                <div id class=card_classes>
                    <div id=header_id class=header_classes>
                        {(header.children)()}
                    </div>
                    <div id=content_id>
                        {(content.children)()}
                    </div>
                </div>
            }.into_any()
        },
        (Some(header), Some(footer)) => {
            let header_id = format!("{id}-header");
            let footer_id = format!("{id}-footer");
            view!{
                <div id class=card_classes>
                    <div id=header_id class=header_classes>
                        {(header.children)()}
                    </div>
                    <div id=content_id>
                        {(content.children)()}
                    </div>
                    <div id=footer_id>
                        {(footer.children)()}
                    </div>
                </div>
            }.into_any()
        },
    }
}