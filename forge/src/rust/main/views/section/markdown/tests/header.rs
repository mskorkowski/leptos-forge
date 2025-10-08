//! Tests for header

use crate::views::section::markdown::MarkdownParser;
use crate::views::section::markdown::MarkdownToken;


/// test to check if header in the first line of Markdown code will be detected
#[test]
fn md_header_in_the_first_line() {
    let markdown = 
r####"## Kaboom"####;
    let header1_level = 2;
    let header1_text = " Kaboom";
    let expected = vec![
        MarkdownToken::Header{
            level: header1_level,
            text: header1_text, 
            len: header1_text.len() + header1_level
        },
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// test to check if header in the second line when first line it empty works
#[test]
fn md_header_in_the_second_line_1() {
    let markdown = 
r####"
## Kaboom"####;
    let header1_level = 2;
    let header1_text = " Kaboom";
    let expected = vec![
        MarkdownToken::Header{
            level: header1_level,
            text: header1_text, 
            len: header1_text.len() + header1_level + 1
        },
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// test to check if header in the second line when first line contains only 
/// whitespace characters (spaces) works
#[test]
fn md_header_in_the_second_line_2() {
    let markdown = 
r####"    
## Kaboom"####;
    let markdown1_text = "    ";
    let header1_level = 2;
    let header1_text = " Kaboom";
    let expected = vec![
        MarkdownToken::Markdown { 
            text: markdown1_text, 
            len: markdown1_text.len(), 
        },
        MarkdownToken::Header{
            level: header1_level,
            text: header1_text, 
            len: header1_text.len() + header1_level + 1
        },
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(markdown);

    assert_eq!(result, expected);
}

/// test to check if header in the second line when first line contains only 
/// whitespace characters (tabs) works
#[test]
fn md_header_in_the_second_line_3() {
    let markdown = "\t\t\t\t\n## Kaboom".to_string();

    let markdown1_text = "\t\t\t\t".to_string();
    let header1_level = 2;
    let header1_text = " Kaboom";
    let expected = vec![
        MarkdownToken::Markdown { 
            text: &markdown1_text, 
            len: markdown1_text.len(), 
        },
        MarkdownToken::Header{
            level: header1_level,
            text: header1_text, 
            len: header1_text.len() + header1_level + 1
        },
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(&markdown);

    assert_eq!(result, expected);
}

/// test to check if header in the second line when first line contains some text
/// whitespace characters (tabs) works
#[test]
fn md_header_in_the_second_line_4() {
    let markdown = "blah blah blah\n## Kaboom".to_string();

    let markdown1_text = "blah blah blah";
    let header1_level = 2;
    let header1_text = " Kaboom";
    let expected = vec![
        MarkdownToken::Markdown { 
            text: markdown1_text, 
            len: markdown1_text.len(), 
        },
        MarkdownToken::Header{
            level: header1_level,
            text: header1_text, 
            len: header1_text.len() + header1_level + 1
        },
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(&markdown);

    assert_eq!(result, expected);
}

/// test to check if header in the header is between two paragraphs
/// whitespace characters (tabs) works
#[test]
fn md_header_in_the_middle_of_the_text() {
    let markdown = "blah blah blah\n## Kaboom\n bla bla bla".to_string();

    let markdown1_text = "blah blah blah";
    let header1_level = 2;
    let header1_text = " Kaboom";
    let markdown2_text = "\n bla bla bla";
    let expected = vec![
        MarkdownToken::Markdown { 
            text: markdown1_text, 
            len: markdown1_text.len(), 
        },
        MarkdownToken::Header{
            level: header1_level,
            text: header1_text, 
            len: header1_text.len() + header1_level + 1
        },
        MarkdownToken::Markdown{ 
            text: markdown2_text, 
            len: markdown2_text.len(), 
        }
    ];

    let parser = MarkdownParser::new();
    let result = parser.parse(&markdown);

    assert_eq!(result, expected);
}

mod internal {
    //! Tests for internal `header` function

    use crate::views::section::markdown::header;
    use crate::views::section::markdown::MarkdownToken;


    /// checks if it finds a header at the beginning of the line in the initial call
    #[test]
    fn header_at_the_start_initial() {
        let markdown = 
        r####"## Kaboom"####;
        let header1_level = 2;
        let header1_text = " Kaboom";
        let expected = Some(
            MarkdownToken::Header{
                level: header1_level,
                text: header1_text, 
                len: header1_text.len() + header1_level
            },
        );

        let result = header(markdown, true);

        assert_eq!(result, expected);
    }    

    /// checks if it finds a header after new line character in initial call
    #[test]
    fn header_after_new_line_initial() {
        let markdown = 
        r####"
## Kaboom"####;
        let header1_level = 2;
        let header1_text = " Kaboom";
        let expected = Some(
            MarkdownToken::Header{
                level: header1_level,
                text: header1_text, 
                len: header1_text.len() + header1_level + 1 // +1 for new line character
            },
        );

        let result = header(markdown, true);

        assert_eq!(result, expected);
    }    

    /// checks if it finds a header at the beginning of the line
    #[test]
    fn header_at_the_start() {
        let markdown = 
        r####"## Kaboom"####;
        // let header1_level = 2;
        // let header1_text = " Kaboom";
        let expected = None;

        let result = header(markdown, false);

        assert_eq!(result, expected);
    }    

    /// checks if it finds a header after new line character
    #[test]
    fn header_after_new_line() {
        let markdown = 
        r####"
## Kaboom"####;
        let header1_level = 2;
        let header1_text = " Kaboom";
        let expected = Some(
            MarkdownToken::Header{
                level: header1_level,
                text: header1_text, 
                len: header1_text.len() + header1_level + 1 // +1 for new line character
            },
        );

        let result = header(markdown, false);

        assert_eq!(result, expected);
    }

    /// Empty header at the start of the document
    #[test]
    fn empty_header_initial() {
        let markdown = 
        r####"##
"####;
        let header1_level = 2;
        let header1_text = "";
        let expected = Some(
            MarkdownToken::Header{
                level: header1_level,
                text: header1_text, 
                len: header1_text.len() + header1_level
            },
        );

        let result = header(markdown, true);

        assert_eq!(result, expected);
    }

    /// Empty header in the second line
    #[test]
    fn empty_header() {
        let markdown = 
        r####"
##
"####;
        let header1_level = 2;
        let header1_text = "";
        let expected = Some(
            MarkdownToken::Header{
                level: header1_level,
                text: header1_text, 
                len: header1_text.len() + header1_level + 1 // +1 for new line character
            },
        );

        let result = header(markdown, false);

        assert_eq!(result, expected);
    }
}
