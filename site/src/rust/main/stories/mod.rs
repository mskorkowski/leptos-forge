//! Main entrypoint to the `leptos_forge` site
mod components;
mod setup;

use components::Components;
use setup::Setup;
use forge::RouteDef;
use forge::Section;

/// description of the [Components] section
const MAIN_DESCRIPTION: &str = r############"
# Leptos_forge

`leptos_forge` is a library that is meant to help you create awesome and reliable components in Rust and Leptos. 

The most recent documentation is available at [leptos_forge site](https://mskorkowski.github.io).

> **Warning**
>
> Leptos Forge is still at the early stages of development and is really bare-bone project. Many of the developer experience features
> are not yet implemented.

## Features

- **Trivial integration with Leptos** - You can use Leptos components directly in your Forge stories without any additional setup
- **Customizable** - It's easy to add new feature and customize almost any aspect of the resulting application to fit your needs
- **Descriptive** - Leptos Forge allows you to write your descriptions using Markdown
- **Rust API** - This project was created with programming Rust in mind to make it easy to write your stories

## Leptos Forge vs Storybook

[Storybook](https://storybook.js.org/) is a popular tool used to create frontend components in isolation and test them and was a big 
inspiration for Leptos Forge. However, Leptos Forge has some unique features that make it different from Storybook:

| Feature | Leptos Forge | Storybook |
|:--------|:------------:|:---------:|
| Programming language | Rust | JavaScript/TypeScript |
| Targeted frameworks | Leptos | React, Vue, Angular and many more |
| Usage | It's a library around which you write your stories | It's an application that manages your stories |
| Extending | Since you control the entire application, you can easily extend it to fit your needs | Storybook has a lot's of features and extensions to meet your needs but if you need something not on that list, it's not so easy to add it yourself |
| Look | Ugly, let's be frank | It's clean and nice to look at |
| Which should I use? | If you are using Rust and Leptos, you might want to consider Leptos Forge. You must remember it's still a bleeding edge project. | If Storybook meets your needs, go there. It's mature, stable and supported by a lots of people |

"############;


/// Main page for the leptos_forge site
#[derive(Debug, Default, Clone, Copy)]
struct Main;

impl Section for Main {
    fn description(&self) -> &'static str {
        MAIN_DESCRIPTION
    }
}

/// Top level routes for the leptos_forge site
pub const ROUTES: &[RouteDef] = &[
    RouteDef::section::<Main>("/", "Leptos Forge", &[]),
    RouteDef::section::<Setup>("setup", "Setup", &[]),
    RouteDef::section::<Components>("components", "Control Panel Components", components::ROUTES),
];