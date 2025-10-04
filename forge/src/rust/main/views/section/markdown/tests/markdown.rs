//! Tests for Markdown parsing which are not using other custom components
//! which are bit more complex

use crate::views::section::markdown::MarkdownParser;
use crate::views::section::markdown::MarkdownToken;

/// Test the parser for a multiline paragraph at the start of Markdown document
#[test]
fn md_multiline_paragraph_at_the_start_of_document() {
  let markdown: &str = r############"Blah blah blah
more blah blah blah"############;

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

/// Test the parser for an inline code at the start of Markdown document
#[test]
fn md_inline_code_at_the_beginning_of_document() {
  let markdown: &str = r############"`inline code`"############;

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

/// Test the parser for an inline code at the start of 2nd line of Markdown document
#[test]
fn md_inline_code_at_the_beginning_of_the_line() {
  let markdown: &str = r############"
`inline code`"############;

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

/// Test the parser for an inline code in the middle of the line
#[test]
fn md_inline_code_at_the_middle_of_the_line() {
  let markdown: &str = r############"Blah blah `inline code` blah blah"############;

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

/// Test the parser for an inline code at the end of the line
#[test]
fn md_inline_code_at_the_end_of_the_line() {
  let markdown: &str = r############"Blah blah `inline code`
blah blah"############;

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

/// Test the parser for an inline code at the end of the document
#[test]
fn md_inline_code_at_the_end_of_the_document() {
  let markdown: &str = r############"Blah blah `inline code`"############;

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
