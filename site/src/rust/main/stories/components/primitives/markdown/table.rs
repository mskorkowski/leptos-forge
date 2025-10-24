//! Story about GitHub flavored tables
//!

use leptos::prelude::*;

use ui_components::primitives::markdown::Markdown;
use ui_components::widgets::field::Codearea;
use utils_leptos::signal::URwSignal;

use forge::Story;

/// Description of the label primitive
const MARKDOWN_DESC: &str = r############"
# Markdown
# tables
 
This story shows the usage of the GitHub flavored tables extension to a cmark. 
Tables are used when you would like to show a user a comparison between two or more items or
more then a few similar records.

"############;

/// Sample markdown document
const MARKDOWN_DEMO: &str = r############"
# Markdown 
# tables

This story shows the GitHub flavored tables extension to a cmark.

## Github flavored tables example

```markdown
| Column 1 align left  | Column 2 align center | Column 3 align right |
|:---------------------|:---------------------:|---------------------:|
| Row 1                | Data                  | Data in column 3     |
| Row 2                | More data             | More date in column 3|
```

| Column 1 align left  | Column 2 align center | Column 3 align right |
|:---------------------|:---------------------:|---------------------:|
| Row 1                | Data                  | Data in column 3     |
| Row 2                | More data             | More date in column 3|

## Creating a table

You can create tables with pipes `|` and hyphens `-`. Hyphens are used to create each column's header, while pipes separate each column. You must include a blank line before your table in order for it to correctly render.

```
| First Header  | Second Header |
| ------------- | ------------- |
| Content Cell  | Content Cell  |
| Content Cell  | Content Cell  |
```

| First Header  | Second Header |
| ------------- | ------------- |
| Content Cell  | Content Cell  |
| Content Cell  | Content Cell  |

The pipes on either end of the table are optional.

Cells can vary in width and do not need to be perfectly aligned within columns. There must be at least three hyphens in each column of the header row.

```
| Command | Description |
| --- | --- |
| git status | List all new or modified files |
| git diff | Show file differences that haven't been staged |
```

| Command | Description |
| --- | --- |
| git status | List all new or modified files |
| git diff | Show file differences that haven't been staged |

## Formatting content within your table

You can use formatting such as links, inline code blocks, and text styling within your table:

```
| Command | Description |
| --- | --- |
| `git status` | List all *new or modified* files |
| `git diff` | Show file differences that **haven't been** staged |
```

| Command | Description |
| --- | --- |
| `git status` | List all *new or modified* files |
| `git diff` | Show file differences that **haven't been** staged |

You can align text to the left, right, or center of a column by including colons `:` to the left, right, or on both sides of the hyphens within the header row.

```
| Left-aligned | Center-aligned | Right-aligned |
| :---         |     :---:      |          ---: |
| git status   | git status     | git status    |
| git diff     | git diff       | git diff      |
```

| Left-aligned | Center-aligned | Right-aligned |
| :---         |     :---:      |          ---: |
| git status   | git status     | git status    |
| git diff     | git diff       | git diff      |

To include a pipe `|` as content within your cell, use a `\` before the pipe:

```
| Name     | Character |
| ---      | ---       |
| Backtick | `         |
| Pipe     | \|        |
```

| Name     | Character |
| ---      | ---       |
| Backtick | `         |
| Pipe     | \|        |

## Acknowledgment

Tables documentation stolen from [GFM Tables](https://help.github.com/en/articles/organizing-information-with-tables)

"############;

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct MarkdownTableStory {
    /// text of a markdown document
    text: URwSignal<String>,
}

impl Default for MarkdownTableStory {
    fn default() -> Self {
        let text: URwSignal<String> = URwSignal::new(MARKDOWN_DEMO.to_string());

        Self { text }
    }
}

impl Story for MarkdownTableStory {
    fn view(&self) -> impl IntoView {
        view! {
            <Markdown src=self.text/>
        }
    }

    fn controls(&self) -> impl IntoView {
        view! {
            <Codearea id="leptos-forge-markdown-demo-textarea" text=self.text label={"Markdown document".to_string()} />
        }
    }

    fn description(&self) -> &'static str {
        MARKDOWN_DESC
    }
}
