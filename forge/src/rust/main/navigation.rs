//! Module provides a helpers to create navigation links in the application

use std::fmt::Debug;

use leptos::prelude::*;
use leptos_router::StaticSegment;
use leptos_router::any_nested_route::AnyNestedRoute;
use leptos_router::any_nested_route::IntoAnyNestedRoute;
use leptos_router::components::Route;
use leptos_router::components::RouteProps;
use reactive_stores::Store;
use ui_components::menu::MenuHeader;
use ui_components::menu::MenuState;
use ui_components::menu::Navigate;
use ui_components::primitives::markdown::Markdown;
use utils::prelude::ThreadSafe;

use crate::IntoStory;
use crate::LeptosForgeConfiguration;
use crate::views::story::EmbeddedStory;

use super::Section;
use super::story::Story;
use super::views::section;
use super::views::story::Story;

/// Allows specifying paths of various length (up to 9) and convert it into
/// a [Route] from static iterable tree of [RouteDef]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PathSpec {
    /// Just a root of the path, aka "/"
    Root,
    /// Single level path
    ///
    /// For example: `/level1`
    Level1(&'static str),
    /// Path with two levels
    ///
    /// For example `/home/bathroom`
    Level2(&'static str, &'static str),
    /// Path with three levels
    Level3(&'static str, &'static str, &'static str),
    /// Path with four levels
    Level4(&'static str, &'static str, &'static str, &'static str),
    /// Path with five levels
    Level5(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ),
    /// Path with six levels
    Level6(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ),
    /// Path with seven levels
    Level7(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ),
    /// Path with eight levels
    Level8(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ),
    /// Path with nine levels
    Level9(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    ),
}

impl PathSpec {
    /// Converts a path into a route with given view
    ///
    /// # Arguments
    ///
    /// * view - leptos component
    ///
    /// # Returns
    ///
    /// Returns a route with given path and view
    ///
    pub fn as_route<F>(&self, view: F) -> AnyNestedRoute
    where
        F: Fn() -> AnyView + ThreadSafe + Clone,
    {
        use PathSpec::*;

        match self {
            Root => Route(
                RouteProps::builder()
                    .view(view)
                    .path(StaticSegment("/"))
                    .build(),
            )
            .into_any_nested_route(),
            Level1(seg1) => Route(
                RouteProps::builder()
                    .view(view)
                    .path(StaticSegment(*seg1))
                    .build(),
            )
            .into_any_nested_route(),
            Level2(seg1, seg2) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((StaticSegment(*seg1), StaticSegment(*seg2)))
                    .build(),
            )
            .into_any_nested_route(),
            Level3(seg1, seg2, seg3) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((
                        StaticSegment(*seg1),
                        StaticSegment(*seg2),
                        StaticSegment(*seg3),
                    ))
                    .build(),
            )
            .into_any_nested_route(),
            Level4(seg1, seg2, seg3, seg4) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((
                        StaticSegment(*seg1),
                        StaticSegment(*seg2),
                        StaticSegment(*seg3),
                        StaticSegment(*seg4),
                    ))
                    .build(),
            )
            .into_any_nested_route(),
            Level5(seg1, seg2, seg3, seg4, seg5) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((
                        StaticSegment(*seg1),
                        StaticSegment(*seg2),
                        StaticSegment(*seg3),
                        StaticSegment(*seg4),
                        StaticSegment(*seg5),
                    ))
                    .build(),
            )
            .into_any_nested_route(),
            Level6(seg1, seg2, seg3, seg4, seg5, seg6) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((
                        StaticSegment(*seg1),
                        StaticSegment(*seg2),
                        StaticSegment(*seg3),
                        StaticSegment(*seg4),
                        StaticSegment(*seg5),
                        StaticSegment(*seg6),
                    ))
                    .build(),
            )
            .into_any_nested_route(),
            Level7(seg1, seg2, seg3, seg4, seg5, seg6, seg7) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((
                        StaticSegment(*seg1),
                        StaticSegment(*seg2),
                        StaticSegment(*seg3),
                        StaticSegment(*seg4),
                        StaticSegment(*seg5),
                        StaticSegment(*seg6),
                        StaticSegment(*seg7),
                    ))
                    .build(),
            )
            .into_any_nested_route(),
            Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((
                        StaticSegment(*seg1),
                        StaticSegment(*seg2),
                        StaticSegment(*seg3),
                        StaticSegment(*seg4),
                        StaticSegment(*seg5),
                        StaticSegment(*seg6),
                        StaticSegment(*seg7),
                        StaticSegment(*seg8),
                    ))
                    .build(),
            )
            .into_any_nested_route(),
            Level9(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8, seg9) => Route(
                RouteProps::builder()
                    .view(view)
                    .path((
                        StaticSegment(*seg1),
                        StaticSegment(*seg2),
                        StaticSegment(*seg3),
                        StaticSegment(*seg4),
                        StaticSegment(*seg5),
                        StaticSegment(*seg6),
                        StaticSegment(*seg7),
                        StaticSegment(*seg8),
                        StaticSegment(*seg9),
                    ))
                    .build(),
            )
            .into_any_nested_route(),
        }
    }

    /// Expands the instance of PathSpec with another segment
    ///
    /// For example:
    ///
    /// ```rust
    ///
    /// # use leptos_forge::navigation::PathSpec;
    ///
    /// let root = PathSpec::Root; // root = `/` path in URL
    /// let components = root.extend("components"); // components = `/components` path in URL
    ///
    /// assert_eq!(components, PathSpec::Level1("components"))
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if returned level would need to be above 9
    pub fn extend(&self, next_segment: &'static str) -> Self {
        use PathSpec::*;

        match self {
            Root => Level1(next_segment),
            Level1(seg1) => Level2(seg1, next_segment),
            Level2(seg1, seg2) => Level3(seg1, seg2, next_segment),
            Level3(seg1, seg2, seg3) => Level4(seg1, seg2, seg3, next_segment),
            Level4(seg1, seg2, seg3, seg4) => Level5(seg1, seg2, seg3, seg4, next_segment),
            Level5(seg1, seg2, seg3, seg4, seg5) => {
                Level6(seg1, seg2, seg3, seg4, seg5, next_segment)
            }
            Level6(seg1, seg2, seg3, seg4, seg5, seg6) => {
                Level7(seg1, seg2, seg3, seg4, seg5, seg6, next_segment)
            }
            Level7(seg1, seg2, seg3, seg4, seg5, seg6, seg7) => {
                Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, next_segment)
            }
            Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8) => {
                Level9(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8, next_segment)
            }
            _ => panic!("Nesting above level 9 is not supported"),
        }
    }

    /// Creates a navigation element in the main menu
    pub fn as_navigation_view(
        &self,
        label: &'static str,
        aria_label: Option<&'static str>,
        location: &str,
        store: Store<MenuState>,
    ) -> AnyView {
        use PathSpec::*;

        match self {
            // we are using `border-l-0!` as a hack to remove the left border which looks nicely when we use headers, but not so nice when we don't
            Root => view!{ <Navigate to="/" label aria_label class="ml-6 border-l-0!" location store/> }.into_any(),
            // we are using `border-l-0!` as a hack to remove the left border which looks nicely when we use headers, but not so nice when we don't
            Level1(seg1) => view!{ <Navigate to=format!("/{seg1}") label aria_label class="ml-6 border-l-0!" location store/> }.into_any(),
            Level2(seg1, seg2) => view!{ <Navigate to=format!("/{seg1}/{seg2}") label aria_label class="ml-6 pl-6" location store/> }.into_any(),
            Level3(seg1, seg2, seg3) => view!{ <Navigate to=format!("/{seg1}/{seg2}/{seg3}") label aria_label class="ml-6 pl-9" location store/> }.into_any(),
            Level4(seg1, seg2, seg3, seg4) => view!{ <Navigate to=format!("/{seg1}/{seg2}/{seg3}/{seg4}") label aria_label class="ml-6 pl-12" location store/> }.into_any(),
            Level5(seg1, seg2, seg3, seg4, seg5) => view!{ <Navigate to=format!("/{seg1}/{seg2}/{seg3}/{seg4}/{seg5}") label aria_label class="ml-6 pl-15" location store/> }.into_any(),
            Level6(seg1, seg2, seg3, seg4, seg5, seg6) => view!{ <Navigate to=format!("/{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}") label aria_label class="ml-6 pl-18" location store/> }.into_any(),
            Level7(seg1, seg2, seg3, seg4, seg5, seg6, seg7) => view!{ <Navigate to=format!("/{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}/{seg7}") label aria_label class="ml-6 pl-21" location store/> }.into_any(),
            Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8) => view!{ <Navigate to=format!("/{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}/{seg7}/{seg8}") label aria_label class="ml-6 pl-24" location store/> }.into_any(),
            Level9(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8, seg9) => view!{ <Navigate to=format!("/{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}/{seg7}/{seg8}/{seg9}") label aria_label class="ml-6 pl-27" location store/> }.into_any(),
        }
    }
}

/// Define routes in the application
#[derive(Debug)]
pub enum RouteDef<Data> {
    /// Menu entry which can be navigated
    Route {
        /// path segment in the url
        path: &'static str,
        /// Label in the menu
        label: &'static str,
        /// ARIA label to be set in cases where content of a route link can be confusing
        /// when using assistive technologies such as screen reader.
        /// 
        /// # Example
        /// 
        /// The `leptos_forge` has a button primitive. Assistive technology would
        /// read "button" as it reads the through menu. This can be confusing for
        /// person using screen reader, since it's harder to understand the context
        /// of the link in the menu just from it's title
        aria_label: Option<&'static str>,
        /// component to render when the route is matched
        component: fn(Store<Data>, Store<LeptosForgeConfiguration>,) -> AnyView,
        /// optional children for nested routes
        subroutes: Vec<RouteDef<Data>>,
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
        embedded: fn(data: Store<Data>, view: bool, controls: bool, description: bool) -> AnyView,
        /// Wherever this route should be hidden from the menu/router but still
        /// provide to the story embedding resolution
        ///
        /// hidden entry hides it's all children
        private: bool,
    },
    /// Grouping for a set of routes without any path to be taken
    Header {
        /// path part in the url
        path: &'static str,
        /// Header label in the menu
        label: &'static str,
        /// optional children for nested routes
        subroutes: Vec< RouteDef<Data> >,
    },
}

impl<Data> RouteDef<Data> {
    /// Set the `aria_label` for the route
    pub fn with_aria_label(mut self, aria_label: &'static str) -> Self {
        match self {
            RouteDef::Header{..} => {
                self
            }
            RouteDef::Route { aria_label: ref mut field, .. } => {
                *field = Some(aria_label);
                self
            }
        }
    }
}

impl<Data> Clone for RouteDef<Data> {
    fn clone(&self) -> Self {
        match self {
            Self::Header { path, label, subroutes } => {
                Self::Header{ 
                    path, 
                    label, 
                    subroutes: subroutes.clone(), 
                }
            },
            Self::Route{ 
                path, 
                label, 
                aria_label,
                component, 
                subroutes, 
                embedded, 
                private 
            } => {
                Self::Route{ 
                    path, 
                    label, 
                    aria_label: *aria_label,
                    component: *component, 
                    subroutes: subroutes.clone(), 
                    embedded: *embedded, 
                    private: *private, 
                }
            }
        }
    }
}

impl<Data: 'static> RouteDef<Data> {
    /// Returns the path of the route
    pub fn path(&self) -> &'static str {
        use RouteDef::*;
        match self {
            Route { path, .. } | Header { path, .. } => path,
        }
    }

    /// Returns a list of subroutes for this route
    pub fn subroutes(&self) -> &Vec< RouteDef<Data> > {
        use RouteDef::*;
        match self {
            Route { subroutes, .. } | Header { subroutes, .. } => subroutes,
        }
    }

    /// Extends a prefix path while detecting a "root" path
    ///
    /// # For RouteDef::Route
    ///
    /// Patch which only contains the `/` are considered as root paths
    /// and they return the prefix directly.
    ///
    /// You can use this to create a root path for your routes
    ///
    /// # For RouteDef::Header
    ///
    /// It just returns a `PathSpec` since headers do not contribute to
    /// path
    fn extend(&self, prefix: PathSpec) -> PathSpec {
        use RouteDef::*;
        match self {
            Route { path, .. } | Header { path, .. } => {
                if *path == "/" {
                    prefix
                } else {
                    prefix.extend(path)
                }
            }
        }
    }

    /// Converts the route to list of routes that can be used in `leptos_router`
    pub fn as_routes(&self, prefix: PathSpec, data: Store<Data>, configuration: Store<LeptosForgeConfiguration>, ) -> Vec<AnyNestedRoute> {
        use RouteDef::*;
        match self {
            Route {
                component,
                subroutes,
                ..
            } => {
                let my_path: PathSpec = self.extend(prefix);
                let c = *component;
                let mut routes: Vec<AnyNestedRoute> = vec![
                    my_path.as_route(
                        move || c(data, configuration)
                    )
                ];

                routes.extend(
                    subroutes
                        .iter()
                        .filter(|r| {
                            match r {
                                RouteDef::Route { private, .. } => !private,
                                _ => true, // I seriously don't understand what is should mean, but hey
                            }
                        })
                        .flat_map(|r| r.as_routes(my_path, data, configuration)),
                );

                routes
            }
            Header { subroutes, .. } => {
                let my_path: PathSpec = self.extend(prefix);
                let mut routes: Vec<AnyNestedRoute> = vec![];

                routes.extend(
                    subroutes
                        .iter()
                        .filter(|r| {
                            match r {
                                RouteDef::Route { private, .. } => !private,
                                _ => true, // I seriously don't understand what is should mean, but hey
                            }
                        })
                        .flat_map(|r| r.as_routes(my_path, data, configuration)),
                );

                routes
            }
        }
    }

    /// Builds menu items
    pub fn as_menu_items(
        &self,
        prefix: PathSpec,
        location: &str,
        menu_state: Store<MenuState>,
    ) -> Vec<AnyView> {
        use RouteDef::*;
        match self {
            Route {
                label, 
                aria_label,
                subroutes, ..
            } => {
                let my_path: PathSpec = self.extend(prefix);

                let mut views = vec![my_path.as_navigation_view(label, *aria_label, location, menu_state)];

                views.extend(
                    subroutes
                        .iter()
                        .filter(|r| {
                            match r {
                                RouteDef::Route { private, .. } => !private,
                                _ => true, // I seriously don't understand what is should mean, but hey
                            }
                        })
                        .flat_map(|r| r.as_menu_items(my_path, location, menu_state)),
                );

                views
            }
            Header {
                label, subroutes, ..
            } => {
                let my_path: PathSpec = self.extend(prefix);

                let mut views = vec![
                    (view! {
                        <MenuHeader label class="" />
                    })
                    .into_any(),
                ];

                views.extend(
                    subroutes
                        .iter()
                        .filter(|r| {
                            match r {
                                RouteDef::Route { private, .. } => !private,
                                _ => true, // I seriously don't understand what is should mean, but hey
                            }
                        })
                        .flat_map(|r| r.as_menu_items(my_path, location, menu_state)),
                );

                views
            }
        }
    }

    /// Creates a new page route with a story and it's related sub-stories
    pub fn story<S>(
        path: &'static str,
        label: &'static str,
    ) -> RouteDef<Data> 
    where
        S: 'static + IntoStory<Data = Data> + Default + Copy + ThreadSafe,
    {
        RouteDef::Route {
            path,
            label,
            aria_label: None,
            component: |data, configuration| view! { <Story<S> data configuration /> }.into_any(),
            embedded: |data, view, controls, description| {
                view! {
                    <EmbeddedStory<S> view  controls description data />
                }
                .into_any()
            },
            subroutes: S::default().into_story().subroutes(),
            private: false,
        }
    }

    /// Creates a new private route with a story and it's related sub-stories
    ///
    /// Private story can't be routed into, but still can be embedded into the
    /// section
    ///
    /// It's useful when you need to embed the story in some super generic sections
    /// which needs an interactive example but are not really a story of some
    /// component.
    ///
    /// You can also use it to add custom features for your documentation
    /// which are not supported by the `leptos_forge`.
    pub fn private<S>(
        path: &'static str,
        label: &'static str,
    ) -> RouteDef<Data> 
    where
        S: 'static + IntoStory<Data = Data> + Default + Copy + ThreadSafe,
    {
        RouteDef::Route {
            path,
            label,
            aria_label: None,
            component: |data, configuration| view! { <Story<S> data configuration /> }.into_any(),
            embedded: |data, view, controls, description| {
                view! {
                    <EmbeddedStory<S> view  controls description data />
                }
                .into_any()
            },
            subroutes: S::default().into_story().subroutes(),
            private: true,
        }
    }

    /// Creates a new section route
    ///
    /// Section creates a new Markdown only page, It's intended use is
    /// to group a bunch of related [sections][RouteDef::section]
    /// and [stories][RouteDef::story]
    /// together
    pub fn section<S>(
        path: &'static str,
        label: &'static str,
    ) -> RouteDef<Data> 
    where
        S: 'static + Section<Data=Data> + Default + Copy + Send
    {
        //
        // Remember to update when changed
        //
        // - site `usage/sections` - it contains the documentation on calling this function as
        //   as part of adding the section to your site.
        //

        RouteDef::Route {
            path,
            label,
            aria_label: None,
            component: |data, _| view! { <section::Section<S> data/> }.into_any(),
            embedded: |_, _, _, _| {
                view! { <Markdown src="> Embedding sections is not allowed"  /> }.into_any()
            },
            subroutes: S::default().subroutes(),
            private: false,
        }
    }

    /// Creates a header in the menu
    ///
    /// Header doesn't contribute to the routing path but provides
    /// a visual named separator and header for parts of the group
    /// of routes in the left hand side menu
    pub fn header(path: &'static str, label: &'static str, subroutes: Vec<RouteDef<Data>>) -> RouteDef<Data> {
        RouteDef::Header {
            path,
            label,
            subroutes,
        }
    }
}
