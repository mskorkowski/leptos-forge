//! In this module we have a smoke tests to check if the things work at the 
//! minimum level (aka barely works). Serious tests are in the dedicated
//! modules


use crate::views::section::markdown::MarkdownParser;
use crate::views::section::markdown::MarkdownToken;


/// A smoke test to check if we can discover a markdown code
#[test]
fn md_just_a_markdown() {
    let markdown = "Hello world!";
    let expected = vec![
        MarkdownToken::Markdown{ 
            text: markdown, 
            len: markdown.len()
        }
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// A smoke test to check if we can discover `#`-based headers
#[test]
fn md_just_a_header() {
    let markdown = 
r####"## Kaboom
### More boom"####;
    let header1_level = 2;
    let header1_text = " Kaboom";
    let header2_level = 3;
    let header2_text = " More boom";
    let expected = vec![
        MarkdownToken::Header{
            level: header1_level,
            text: header1_text, 
            len: header1_text.len() + header1_level
        },
        MarkdownToken::Header{
            level: header2_level,
            text: header2_text,
            len: header2_text.len() + header2_level + 1 //for the new line
        }
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// A smoke test to check if we can handle a correctly formatted `<Story />` tag
#[test]
fn md_just_a_story() {
    let markdown = "<Story of=\"aa/bb\" /><Story of='aa/bb'/>";
    let expected = vec![
        MarkdownToken::Story{ 
            story: Some("aa/bb"),
            len: 20, 
        },
        MarkdownToken::Story{ 
            story: Some("aa/bb"),
            len: 19, 
        }
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);
    assert_eq!(result, expected);
}