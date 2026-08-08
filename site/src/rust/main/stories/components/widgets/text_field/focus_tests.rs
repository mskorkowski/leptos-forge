//! Tests for focus interactions with input field

use leptos::prelude::*;

use forge::Play;
use forge::play;
use leptos::tachys::renderer::dom::Node;
use testing_library_dom::SelectorMatcherOptions;
use testing_library_dom::get_by_label_text;
use ui_components::model::Focus;

use super::BasicTextFieldStory;

/// Label to set so we can find the field
const TEXT_FIELD_LABEL: &str = "Test-field-label";

/// Checks if [TextField] properly grabs and releases focus after receiving the
/// signal changes according to the spec
pub fn programmatic_interaction_valid() -> Box<dyn Play<Story=BasicTextFieldStory>> {

    play::<BasicTextFieldStory>("Check the programmatic focus management").
        next(
            "Prepare the field label",
            |_canvas, story| {
                story.label.set(TEXT_FIELD_LABEL.to_string());
                Ok(())
            }
        ).
        next(
            "Make sure that field doesn't have focus", 
            |canvas, story| {
                let focus_state = story.focus.get_untracked();
                if focus_state != Focus::Out {
                    return Err("TextField thinks it somehow have a focus");
                }

                let Some(active_element) = canvas.owner_document().unwrap().active_element() else {
                    // There is no active element on canvas
                    return Ok(());
                };

                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                {
                    let input_node: &Node = input.as_ref();
                    let active_element: &Node = active_element.as_ref();

                    if input_node.is_same_node(Some(active_element)) {
                        let Ok(_) =  input.blur() else {
                            return Err("Can't blur?");
                        };
                    }
                }

                Ok(())
            }
        )
        .next(
            "Set the signal to Focus::Grab",
            |_canvas, story| {
                story.focus.set(Focus::Grab);

                Ok(())
            }
        )
        .next(
            "Check if input has a focus now",
            |canvas, story| {
                let focus_state = story.focus.get_untracked();
                if focus_state != Focus::In {
                    return Err("Looks like TextField thinks it's not focused");
                }

                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                let Some(active_element) = canvas.owner_document().unwrap().active_element() else {
                    return Err("There is no active element");
                };

                {
                    let input: &Node = input.as_ref();
                    let active_element: &Node = active_element.as_ref();

                    if !input.is_same_node(Some(active_element)) {
                        return Err("We didn't grab the focus");
                    }
                }

                Ok(())
            }
        )
        .next(
            "Set the signal to Focus::Blur",
            |_canvas, story| {
                story.focus.set(Focus::Blur);

                Ok(())
            }
        ).
        next(
            "Verify that focus was lost", 
            |canvas, story| {
                let focus_state = story.focus.get_untracked();
                if focus_state != Focus::Out {
                    return Err("TextField thinks it somehow have a focus");
                }

                let Some(active_element) = canvas.owner_document().unwrap().active_element() else {
                    // There is no active element on canvas
                    return Ok(());
                };

                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                {
                    let input_node: &Node = input.as_ref();
                    let active_element: &Node = active_element.as_ref();

                    if input_node.is_same_node(Some(active_element)) {
                        return Err("TextField still has a focus");
                    }
                }

                Ok(())
            }
        )
        .into()

}


/// Checks if [TextField] properly grabs and releases focus after receiving the
/// signal changes according to the spec
pub fn human_interaction_valid() -> Box<dyn Play<Story=BasicTextFieldStory>> {

    play::<BasicTextFieldStory>("Check manual focus management").
        next(
            "Prepare the field label",
            |_canvas, story| {
                story.label.set(TEXT_FIELD_LABEL.to_string());
                Ok(())
            }
        ).
        next(
            "Make sure that field doesn't have focus", 
            |canvas, story| {
                let focus_state = story.focus.get_untracked();
                if focus_state != Focus::Out {
                    return Err("TextField thinks it somehow have a focus");
                }

                let Some(active_element) = canvas.owner_document().unwrap().active_element() else {
                    // There is no active element on canvas
                    return Ok(());
                };

                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                {
                    let input_node: &Node = input.as_ref();
                    let active_element: &Node = active_element.as_ref();

                    if input_node.is_same_node(Some(active_element)) {
                        let Ok(_) =  input.blur() else {
                            return Err("Can't blur?");
                        };
                    }
                }

                Ok(())
            }
        )
        .next(
            "Set the signal to Focus::Grab",
            |canvas, _story| {
                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                let Ok(_) = input.focus() else {
                    return Err("Can't focus the field");
                };

                Ok(())
            }
        )
        .next(
            "Check if input has a focus now",
            |canvas, story| {
                let focus_state = story.focus.get_untracked();
                if focus_state != Focus::In {
                    return Err("Looks like TextField thinks it's not focused");
                }

                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                let Some(active_element) = canvas.owner_document().unwrap().active_element() else {
                    return Err("There is no active element");
                };

                {
                    let input: &Node = input.as_ref();
                    let active_element: &Node = active_element.as_ref();

                    if !input.is_same_node(Some(active_element)) {
                        return Err("We didn't grab the focus");
                    }
                }

                Ok(())
            }
        )
        .next(
            "Set the signal to Focus::Blur",
            |canvas, _story| {
                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                let Ok(_) = input.blur() else {
                    return Err("Can't focus the field");
                };

                Ok(())
            }
        ).
        next(
            "Verify that focus was lost", 
            |canvas, story| {
                let focus_state = story.focus.get_untracked();
                if focus_state != Focus::Out {
                    return Err("TextField thinks it somehow have a focus");
                }

                let Some(active_element) = canvas.owner_document().unwrap().active_element() else {
                    // There is no active element on canvas
                    return Ok(());
                };

                let Ok(input) = get_by_label_text(canvas, TEXT_FIELD_LABEL, SelectorMatcherOptions::default()) else {
                    return Err("Can't find the text field");
                };

                {
                    let input_node: &Node = input.as_ref();
                    let active_element: &Node = active_element.as_ref();

                    if input_node.is_same_node(Some(active_element)) {
                        return Err("TextField still has a focus");
                    }
                }

                Ok(())
            }
        )
        .into()

}