//! Trait for grouping the [stories][Story] together
//! 

use crate::RouteDef;

/// Description of the [Section] shown when you don't implement a description
/// 
/// When you add section into your `leptos_forge` application it will show up in the UI
/// as the description.
const SECTION_DESC: &str = r#############"
# How to implement a section

Sections are intended to wrap a multiple stories in the book under single umbrella


## Implementing

Your section is just a description of the given part of the Book. To construct the section you must use the `Default` trait.
But since your section will never have any data, you can safely derive it.

```rust

#[derive(Default, Clone, Copy)]
struct MySection;
```

### Implementing `Section` trait

Create a description for your section using a Markdown syntax. The description should explain the goals of the section.
The easiest way to implement it is by creating a `const &str`. For example this description is created using

```rust

const SECTION_DESC: &str = r############"
# Section";

Sections are intended to wrap a multiple stories in the book under single umbrella.

...

"############
```

Notice that we use a lot (12 `#` in the example) which should be above whatever you want in your Markdown code where
headers are up to `6` levels.

After creating the constant with description you implement the `Section::description` method which should just return this constant.

```rust

const SECTION_DESC: &str = ...

impl Section for MySection {
    fn description(&self) -> &'static str {
       SECTION_DESC
    }
}
```

"#############;

/// Section describing big chunk of the UI components
/// 
/// Sections contain only Markdown text.
pub trait Section: Default { 
    // 
    //  Things to update when changing the API
    //
    //  - **`usage/section`** - document the usage of the Section on the site.
    // 


    /// Function returns the text of the section
    fn description(&self) -> &'static str {
        SECTION_DESC
    }

    /// Returns a list of subroutes for the Section
    fn subroutes(&self) -> Vec<RouteDef> {
        vec![]
    }
}