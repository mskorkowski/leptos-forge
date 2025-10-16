//! Section about sections in `leptos_forge`

use forge::Section;

/// Content of the Sections section
const SECTIONS: &str = r############"
# Sections

When you would like to add the chapter containing a lot of text describing
the design choices, or what to use the components in sub chapters, or in
general organize your `leptos_forge` better you can use sections. For 
example this chapter is a section itself.

## Creating a section

Section must implement the `Default` trait. In most of the cases you will
be able to derive it.

To create a new section you need to define a new struct that implements the
`leptos_forge::Section` trait.

| Function                                | Description                            | Default behavior                            |
|:----------------------------------------|:---------------------------------------|---------------------------------------------|
| `fn description(&self) -> &'static str` | Returns a Markdown formatted string with the content of this section | Returns "How to implement a section" guide |
| `fn subroutes(&self) -> Vec<RouteDef>`  | Returns a vector with stories and sections under this section        | Returns an empty vector |

Example:

```rust

use leptos_forge::Section;

#[derive(Default, Clone, Copy)]
pub struct MySection;

impl Section for MySection {
    fn description(&self) -> &'static str {
       "This section is about our awesome component ..."
    }
}

```

> [!TIP]
>
> While using the `leptos_forge` I found that the most readable way of writing the 
> section is to create a `const &str` which later is just returned by the
> `description` function.
>
> This looks like this:
>
> ```rust
>
> const MY_SECTION: &'static str = r########"
> # My Section
> 
> This section is about our awesome component ...
>
> "########;
>
> impl Section for MySection {
>    fn description(&self) -> &'static str {
>       MY_SECTION
>    }
> }
>
> ```

## Adding stories and subsections

To add stories and section under the section you must implement the `subroutes`
method. 

Example:

Assuming that there is a story `ComponentStory` and a section `TranslatingComponentSection`.
To add them

```rust
use leptos_forge::Section;
use leptos_forge::RouteDef;

#[derive(Default, Clone, Copy)]
pub struct MySection;

impl Section for MySection {
    ...

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::story::<ComponentStory>("component", "Component"),        
            RouteDef::section::<TranslatingComponentSection>("translation", "Translation"),
        ]
    }

}
```

You can read more about this in the [Routing](http://localhost:8000/documentation/routes) section.

## Embedding stories

Syntax:

```
<Story of="path/to/the/substory" controls/>
```

| Attribute | Value | Description |
|:-----------|:---------|:----------------|
| of | string | Path to the substory, relative to the section where you embed the story. It's not possible to "go up in sections tree" |
| controls | bool | Boolean attribute. If present it will enable rendering of the embedded control panel |

### Limitations of the `controls` attribute

Only supported syntax for the `controls` attribute is

```
<Story ... controls />
```

We don't support `controls=""` or other forms of the boolean attributes in html.

The progress can be tracked in the [GH issue](https://github.com/mskorkowski/leptos-forge/issues/62?issue=mskorkowski%7Cleptos-forge%7C73)

### Example

```rust
use leptos_forge::Section;
use leptos_forge::Story;
use leptos_forge::RouteDef;

#[derive(Default, Clone, Copy)]
pub struct ComponentStory;

impl Story for ComponentStory {
    ...
}

const MY_SECTION: &'static str = r########"
# My Section

To embed the ComponentStory you should write

<Story of="component" />

"########;

#[derive(Default, Clone, Copy)]
pub struct MySection;

impl Section for MySection {
    fn description(&self) -> &'static str {
    }

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::story::<ComponentStory>("component", "Component"),
        ]
    }

}
```

"############;

/// Section about sections in `leptos_forge`
#[derive(Debug, Default, Clone, Copy)]
pub struct SectionsSection;

impl Section for SectionsSection {
    fn description(&self) -> &'static str {
        SECTIONS
    }
}
