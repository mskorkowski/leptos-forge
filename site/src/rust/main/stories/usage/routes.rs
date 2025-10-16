//! Module containing the section about `ROUTES` in the `leptos_forge` app.
//! 

use forge::Section;


/// URwSignal description
const ROUTING: &str = r############"
# Routing

`leptos_forge` uses `leptos_forge::RouteDef` struct to manage all of the routing
information for the specific path.

There are four standard ways to create a `RouteDef` instance.

1. `RouteDef::story` - to create a story entry
2. `RouteDef::section` -  to create a section entry in your storybook
3. `RouteDef::private` - to create a nonnavigable routes to some stories
4, `RouteDef::header` - to create a header in your menu

## `RouteDef::story`

Story describes the component under a special state. You can read more about it
in [Story](/documentation/story) section.

This function creates a new instance of `RouteDef` which points towards a `Story`.

```rust
use leptos_forge::Story;
use leptos_forge::RouteDef;

#[derive(Default, Clone, Copy)]
struct MyStory {
    ...
}

impl Story for MyStory {
    ...
}

RouteDef::story::<MyStory>("my_story_path", "My Story Menu Title");
```

The invocation of the `RouteDef::story` function looks slightly nonstandard
because it takes a type argument pointing to the .

The arguments from the example

| Argument | Description |
|:---------|:------------|
| `my_stroy_path` | It's a segment of the url path which will be added to your section or story path |
| `My Story Menu Title` | It's what will be shown on the right hand side menu |

### Custom initial state

If you would like to reuse the story with different initial state, you can do so
implementing the `IntoStory` trait. The example below shows how to add this
special case as the sub-story

```rust
#[derive(Default, Clone, Copy)]
struct MyStory {
    ...
}

impl Story for MyStory {
    ...

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::story::<SpecialMyStoryCase>("special_case", "Special Case")
        ]
    }
}

#[derive(Default, Clone, Copy)]
struct SpecialMyStoryCase;

impl IntoStory for SpecialMyStoryCase {
    type Story = MyStory;

    fn into_story(self) -> Self::Story {
        ...
    }
}

```

## `RouteDef::section`

Section is a page describing something. In most cases it describes a component
and all associated stories. You can read more about them in the 
[Section's documentation](/documentation/section).

The example below shows how to create a section related `RouteDef`.

```rust
use leptos_forge::Section;
use leptos_forge::RouteDef;

#[derive(Default, Clone, Copy)]
struct MySection {
    ...
}

impl Section for MySection {
    ...
}

RouteDef::section::<MySection>("my_section_path", "My Section Menu Title");
```

### Embedding stories

Sections ability to embed the stories is related to the routing information.
When you write your story.

```rust
use leptos_forge::Story;
use leptos_forge::Section;
use leptos_forge::RouteDef;

#[derive(Default, Clone, Copy)]
struct MySubStory;

impl Story for MySubStory {}


#[derive(Default, Clone, Copy)]
struct MyStory;

impl Story for MyStory {
    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::story<MySubStory>("my_sub_story", "MySubStory")
        ]
    }
}

const MY_SECTION: &'static str = r########"
# My Section

To embed the `MySubStory` for `MySection` you should write

<Story of="my_story/my_sub_story" />

"########;

#[derive(Default, Clone, Copy)]
struct MySection {
    ...
}

impl Section for MySection {
    ...

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::story<MyStory>("my_story", "MyStory")
        ]
    }
}

```

## `RouteDef::private`

This builder function is useful when you would like to embed a story with a given
state into the section but for some reason it should not be a story for itself like

- Example showcasing usage context outside of the component
- Example referencing other component to show alternate solution in some cases

Any section or story which is subroute of the `private` will not be shown in the
menu and will not be routable. 

Embedding of the stories on the other hand will work as normal.

## Defining routes in stories and sections

Both traits `Story` and `Section` have a method `fn subroutes(&self) -> Vec<RouteDef>`.
By default this method returns an empty vector.

If you implement the method you can add a subroutes to your sections and stories.

For example the [Documentation - Story](http://localhost:8000/documentation/story) at the
time of writing defined a `testing` subroute like this

```rust
#[derive(Debug, Default, Clone, Copy)]
pub struct StorySection;

impl Section for StorySection {
    ...

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::section::<TestingSection>("testing", "Testing"),
        ]
    }
}
```

## Defining routes for the `leptos_forge::App`

The `leptos_forge::App` component takes a `routes: Vec<RouteDef>` as an argument. This
are the highest level routes.

At the time of writing the `leptos_forge` site used

```rust
// Top level routes for the leptos_forge site
    let routes = vec![
        RouteDef::section::<Main>("/", "Leptos Forge"),
        RouteDef::header("guides", "GUIDES", vec![
            RouteDef::section::<Setup>("create_project", "Create project"),
            RouteDef::section::<RefineCounterStory>("first_story", "Implement the first story"),
            RouteDef::section::<AddingTests>("adding_tests", "Adding tests"),
            RouteDef::section::<Resources>("resources", "Resources"),
            RouteDef::section::<Tailwind>("tailwind", "Tailwind"),
            RouteDef::section::<Nix>("nix", "Nix"),
        ]),
        RouteDef::header("documentation", "DOCUMENTATION", vec![
            RouteDef::section::<StorySection>("story", "Story"),
            RouteDef::section::<SectionsSection>("section", "Section"),
            RouteDef::section::<RoutesSection>("routes", "Routing"),
            RouteDef::section::<URwSignalSection>("urwsignal", "URwSignal"),
        ]),
        RouteDef::header("development", "DEVELOPMENT", vec![
            RouteDef::section::<Components>("components", "Components")
        ])
    ];

    mount_to_body(move || {
        view!{
            <App routes logo="/resources/leptos_forge/logo/logo.svg" />
        }
    });
```

"############;


/// Section about `Routes` and [RouteDef][forge::RouteDef]
#[derive(Debug, Default, Clone, Copy)]
pub struct RoutesSection;

impl Section for RoutesSection {
    fn description(&self) -> &'static str {
        ROUTING
    }
}