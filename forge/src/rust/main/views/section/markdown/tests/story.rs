//! Tests related to parsing a `<Story />` tag
//!

use crate::views::section::markdown::MarkdownParser;
use crate::views::section::markdown::MarkdownToken;

/// Single story test
#[test]
fn md_single_story() {
    let markdown = r####"<Story of="path" />"####;
    let expected = vec![MarkdownToken::Story {
        story: Some("path"),
        len: markdown.len(),
        controls: false,
    }];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// Story split using new line
#[test]
fn md_single_story_with_new_line() {
    let markdown = r####"<Story 
  of="path"
/>"####;
    let expected = vec![MarkdownToken::Story {
        story: Some("path"),
        len: markdown.len(),
        controls: false,
    }];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// Story with controls at the end of tag content
#[test]
fn md_single_story_with_controls_at_the_end_of_tag_content() {
    let markdown = r####"<Story of="path" controls/>"####;
    let expected = vec![MarkdownToken::Story {
        story: Some("path"),
        len: markdown.len(),
        controls: true,
    }];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// Story with controls at the beginning of tag content
#[test]
fn md_single_story_with_controls_at_the_beginning_of_tag_content() {
    let markdown = r####"<Story controls of="path"/>"####;
    let expected = vec![MarkdownToken::Story {
        story: Some("path"),
        len: markdown.len(),
        controls: true,
    }];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// Story with controls surrounded by spaces
#[test]
fn md_single_story_with_controls_surrounded_by_spaces_at_the_end() {
    let markdown = r####"<Story of="path" controls />"####;
    let expected = vec![MarkdownToken::Story {
        story: Some("path"),
        len: markdown.len(),
        controls: true,
    }];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// Story with controls surrounded by spaces
#[test]
fn md_single_story_with_new_lines() {
    let markdown = r####"<Story
of="path"
controls
/>"####;
    let expected = vec![MarkdownToken::Story {
        story: Some("path"),
        len: markdown.len(),
        controls: true,
    }];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// Story where controls is not present because `controlscontrols` is not
/// a `controls` attribute
#[test]
fn md_single_story_without_controls_due_to_invalid_attribute_name() {
    let markdown = r####"<Story
of="path"
controlscontrols
/>"####;
    let expected = vec![MarkdownToken::Story {
        story: Some("path"),
        len: markdown.len(),
        controls: false,
    }];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

mod internal {
    use crate::views::section::markdown::MarkdownToken;
    use crate::views::section::markdown::OfAttribute;
    use crate::views::section::markdown::find_of_attribute;
    use crate::views::section::markdown::parse_of_attribute;
    use crate::views::section::markdown::story;

    // Single story test
    #[test]
    fn single_story() {
        let markdown = r####"<Story of="path" />"####;
        let expected = Some(MarkdownToken::Story {
            story: Some("path"),
            len: markdown.len(),
            controls: false,
        });

        let result = story(markdown);

        assert_eq!(result, expected);
    }

    /// Test Searches for the of attribute which is at the start of tag content
    #[test]
    fn find_of_attribute_at_0() {
        let tag = r#"of="path" "#;
        let expected = Some("path");

        let result = find_of_attribute(tag);

        assert_eq!(result, expected);
    }

    /// Check actual parsing of the `of` attribute from the tag content
    #[test]
    fn parse_of_attribute_at_0() {
        let tag = r#"of="path" "#;
        let expected = Some(OfAttribute { subpath: "path" });

        let result = parse_of_attribute(tag);

        assert_eq!(result, expected);
    }
}
