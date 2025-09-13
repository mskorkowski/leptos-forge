//! Story about GitHub flavored admonishes
//! 

use leptos::prelude::*;

use ui_components::primitives::markdown::Markdown;
use ui_components::widgets::field::Codearea;

use forge::Story;

use utils_leptos::signal::URwSignal;

/// Description of the label primitive
const MARKDOWN_DESC: &str = r############"
# Markdown
# admonishes
 
Admonishes are used to add extra context to the content or to highlight important points.

"############;

/// Sample markdown document
const MARKDOWN_DEMO: &str = r############"
# Markdown 
# admonishes

This story shows the GitHub flavored admonishes extension to a cmark.

## Github flavored admonishes

GitHub flavored admonishes are a way to highlight important information in your markdown document. They look like this:

### Note

```markdown
> [!NOTE]  
> Highlights information that users should take into account, even when skimming.
```

> [!NOTE]  
> Highlights information that users should take into account, even when skimming.

### Tip

```markdown
> [!TIP]
> Optional information to help a user be more successful.
```

> [!TIP]
> Optional information to help a user be more successful.

### Important

```markdown
> [!IMPORTANT]  
> Crucial information necessary for users to succeed.
```

> [!IMPORTANT]  
> Crucial information necessary for users to succeed.

### Warning

```markdown
> [!WARNING]  
> Critical content demanding immediate user attention due to potential risks.
```

> [!WARNING]  
> Critical content demanding immediate user attention due to potential risks.

### Caution

```markdown
> [!CAUTION]
> Negative potential consequences of an action.
```

> [!CAUTION]
> Negative potential consequences of an action.

"############;

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct MarkdownAdmonishStory {
    /// text of a markdown document
    text: URwSignal<String>
}

impl Default for MarkdownAdmonishStory {
    
    fn default() -> Self {
        let text: URwSignal<String> = URwSignal::new(MARKDOWN_DEMO.to_string());

        Self{  
            text
        }
    }
}

impl Story for MarkdownAdmonishStory {
    fn view(&self) -> AnyView {
        view! {
            <Markdown src=self.text/>
        }.into_any()
    }

    fn controls(&self) -> AnyView {
        view!{
            <Codearea id="storybook-markdown-demo-textarea" text=self.text label={"Markdown document".to_string()} />
        }.into_any()
    }

    fn description(&self) -> &'static str {
        MARKDOWN_DESC
    }
}