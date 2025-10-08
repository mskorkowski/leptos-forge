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

| Function                                | Description                            |
|:----------------------------------------|:---------------------------------------|
| `fn description(&self) -> &'static str` | This function returns a Markdown formatted string with the content of the section |


Example:

```rust

use leptos_forge::Section;

#[derive(Default)]
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

## Adding a Section

To add a section to your `ROUTES`, you need to add it. The easiest way is to call 
the `RouteDef::section` function.

`RouteDef::section` takes three arguments:

| Argument    | Type                  | Description                                                |
|:------------|:----------------------|:-----------------------------------------------------------|
| `path`      | `&'static str`        | The last segment of the path leading to the section        |
| `label`     | `&'static str`        | The label shown in the menu that you click                 |
| `subroutes` | `&'static [RouteDef]` | Subroutes that will be added to the menu as submenu items   |

For the `MySection` above, the call could look like:

```rust
use leptos_forge::RouteDef;

const ROUTES: &[RouteDef] = &[
    ...
    RouteDef::section::<MySection>(
        "my_section",    // <- path
        "My Section",    // <- label
        &[
            ...          // <- subroutes
        ])
    ...
];
```

## Embedding stories

Syntax:

`<Story of="path/to/the/substory" controls/>`

| Attribute | Value | Description |
|:-----------|:---------|:----------------|
| of | string | Path to the substory, relative to the section where you embed the story. It's not possible to "go up in sections tree" |
| controls | bool | Boolean attribute. If present it will enable rendering of the embedded controls panel |

"############;


/// Section about sections in `leptos_forge`
#[derive(Debug, Default, Clone, Copy)]
pub struct SectionsSection;

impl Section for SectionsSection {
    fn description(&self) -> &'static str {
        SECTIONS
    }
}