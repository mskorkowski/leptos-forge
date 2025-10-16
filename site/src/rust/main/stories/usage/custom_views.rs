//! Section about going full custom on the view side
//! 


use forge::Section;

/// Describes customizing views
const CUSTOM_VIEWS: &str = r############"
# Customizing `leptos_forge`

If stories and sections are not enough for you, you can add your own kind of
page to the `leptos_forge` by implementing a custom builder for `RouteDef::Route`!

RouteDef::Route looks like this:

```rust
/// Define routes in the application
#[derive(Debug,Clone)]
pub enum RouteDef {
    /// Menu entry which can be navigated
    Route{
        /// path segment in the url
        path: &'static str,
        /// Label in the menu
        label: &'static str,
        /// component to render when the route is matched
        component: fn() -> AnyView,
        /// optional children for nested routes
        subroutes: Vec<RouteDef>,
        /// Returns the view embedded in the section
        /// 
        /// # Arguments
        /// 
        /// - **view** - wherever we should show the canvas.
        ///   
        ///   Current embedding code hard codes this to true since it wouldn't
        ///   make a lot of sense to do otherwise currently due to the fact that
        ///   every embedding is on it's own.
        /// 
        /// - **controls** - wherever we should show controls
        /// - **description** - wherever we should show description of the story
        /// 
        ///   Current version of the Markdown parser doesn't allow to set this
        ///   value.
        /// 
        /// # Embedding in the section
        /// 
        /// Inside the Markdown returned from [Section::description] method
        /// you can add the `<Story />` tag. It has the following boolean attributes
        /// 
        /// - **controls**
        /// 
        /// 
        /// The code below will enable all of the before mentioned attributes
        /// 
        /// ```markdown
        /// 
        /// <Story of="path/to/the/substory" controls />
        /// 
        /// ```
        /// 
        /// 
        embedded: fn(view: bool, controls: bool, description: bool) -> AnyView,
        /// Wherever this route should be hidden from the menu/router but still
        /// provide to the story embedding resolution
        /// 
        /// hidden entry hides it's all children
        private: bool,
    },
    ...
}

```

Custom entry for the menu could be implemented like this

```rust

fn custom_entry() -> RouteDef {
    RouteDef::Route {
        path: "custom_view",
        label: "Custom View",

        component: fn() { view!{ <MyCustomView /> }.into_any() },
        subroutes: vec![],
        embedded: fn() { view!{ <MyCustomEmbeddedView /> }.into_any() },
        private: false,
    }
}

```

`component` function will be called when user navigates via menu entry and
`embedded` will be called in place of story in the section.

Now just add as any other route in your application.

"############;

/// Section about fully custom views
#[derive(Debug, Default, Clone, Copy)]
pub struct CustomViews;

impl Section for CustomViews {
    fn description(&self) -> &'static str {
        CUSTOM_VIEWS
    }
}