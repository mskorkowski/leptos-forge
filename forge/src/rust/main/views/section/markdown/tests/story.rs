//! Tests related to parsing a `<Story />` tag
//! 

use crate::views::section::markdown::MarkdownParser;
use crate::views::section::markdown::MarkdownToken;


/// Single story test
#[test]
fn md_single_story() {
    let markdown = 
r####"<Story of="path" />"####;
    let expected = vec![
        MarkdownToken::Story{ 
            story: Some("path"),
            len: markdown.len(), 
        },
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

mod internal {
    use crate::views::section::markdown::find_of_attribute;
    use crate::views::section::markdown::parse_of_attribute;
    use crate::views::section::markdown::story;
    use crate::views::section::markdown::MarkdownToken;
    use crate::views::section::markdown::OfAttribute;


    // Single story test
    #[test]
    fn single_story() {
        let markdown = 
r####"<Story of="path" />"####;
        let expected = Some(
            MarkdownToken::Story{ 
                story: Some("path"),
                len: markdown.len(), 
            },
        );

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
        let expected = Some(OfAttribute{
            subpath: "path",
        });

        let result = parse_of_attribute(tag);

        assert_eq!(result, expected);
    }
}