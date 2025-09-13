//! Module provides a helpers to create navigation links in the application


use std::fmt::Debug;

use leptos::prelude::*;
use leptos_router::any_nested_route::AnyNestedRoute;
use leptos_router::any_nested_route::IntoAnyNestedRoute;
use leptos_router::components::Route;
use leptos_router::components::RouteProps;
use leptos_router::StaticSegment;
use ui_components::menu::Navigate;
use utils_leptos::signal::ThreadSafe;

use super::story::Story;
use super::views::page::Page;
use super::views::section;
use super::Section;

/// Allows specifying paths of various length (up to 9) and convert it into
/// a [Route] from static iterable tree of [RouteDef]
#[derive(Debug, Clone, Copy)]
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
    Level5(&'static str, &'static str, &'static str, &'static str, &'static str),
    /// Path with six levels
    Level6(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str),
    /// Path with seven levels
    Level7(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str),
    /// Path with eight levels
    Level8(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str),
    /// Path with nine levels
    Level9(&'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str, &'static str),
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
    pub fn as_route(&self, view: fn() -> AnyView) -> AnyNestedRoute {
        use PathSpec::*;

        match self {
            Root => Route(RouteProps::builder().view(view).path(StaticSegment("/")).build()).into_any_nested_route(),
            Level1(seg1,) => Route(RouteProps::builder().view(view).path(StaticSegment(*seg1)).build()).into_any_nested_route(),
            Level2(seg1, seg2 ) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2))).build()).into_any_nested_route(),
            Level3(seg1, seg2, seg3 ) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2), StaticSegment(*seg3))).build()).into_any_nested_route(),
            Level4(seg1, seg2, seg3, seg4 ) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2), StaticSegment(*seg3), StaticSegment(*seg4))).build()).into_any_nested_route(),
            Level5(seg1, seg2, seg3, seg4, seg5) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2), StaticSegment(*seg3), StaticSegment(*seg4), StaticSegment(*seg5))).build()).into_any_nested_route(),
            Level6(seg1, seg2, seg3, seg4, seg5, seg6 ) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2), StaticSegment(*seg3), StaticSegment(*seg4), StaticSegment(*seg5), StaticSegment(*seg6))).build()).into_any_nested_route(),
            Level7(seg1, seg2, seg3, seg4, seg5, seg6, seg7 ) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2), StaticSegment(*seg3), StaticSegment(*seg4), StaticSegment(*seg5), StaticSegment(*seg6), StaticSegment(*seg7))).build()).into_any_nested_route(),
            Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2), StaticSegment(*seg3), StaticSegment(*seg4), StaticSegment(*seg5), StaticSegment(*seg6), StaticSegment(*seg7), StaticSegment(*seg8))).build()).into_any_nested_route(),
            Level9(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8, seg9) => Route(RouteProps::builder().view(view).path((StaticSegment(*seg1), StaticSegment(*seg2), StaticSegment(*seg3), StaticSegment(*seg4), StaticSegment(*seg5), StaticSegment(*seg6), StaticSegment(*seg7), StaticSegment(*seg8), StaticSegment(*seg9))).build()).into_any_nested_route(),
        }
    }

    /// Expands the instance of PathSpec with another segment
    /// 
    /// For example:
    /// 
    /// ```rust
    /// 
    /// # use storybook::app::navigation::PathSpec;
    /// 
    /// 
    /// 
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
            Level5(seg1, seg2, seg3, seg4, seg5) => Level6(seg1, seg2, seg3, seg4, seg5, next_segment),
            Level6(seg1, seg2, seg3, seg4, seg5, seg6) => Level7(seg1, seg2, seg3, seg4, seg5, seg6, next_segment),
            Level7(seg1, seg2, seg3, seg4, seg5, seg6, seg7) => Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, next_segment),
            Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8) => Level9(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8, next_segment),
            _ => panic!("Nesting above level 9 is not supported"),
        }
    }

    /// Creates a navigation element in the main menu
    pub fn as_navigation_view(&self, label: &'static str) -> AnyView {
        use PathSpec::*;

        match self {
            Root => view!{ <Navigate to="/" label=label class="pl-0" /> }.into_any(),
            Level1(seg1) => view!{ <Navigate to=seg1 label=label class="pl-3"/> }.into_any(),
            Level2(seg1, seg2) => view!{ <Navigate to=format!("{seg1}/{seg2}") label=label class="pl-6"/> }.into_any(),
            Level3(seg1, seg2, seg3) => view!{ <Navigate to=format!("{seg1}/{seg2}/{seg3}") label=label class="pl-9"/> }.into_any(),
            Level4(seg1, seg2, seg3, seg4) => view!{ <Navigate to=format!("{seg1}/{seg2}/{seg3}/{seg4}") label=label class="pl-12"/> }.into_any(),
            Level5(seg1, seg2, seg3, seg4, seg5) => view!{ <Navigate to=format!("{seg1}/{seg2}/{seg3}/{seg4}/{seg5}") label=label class="pl-15"/> }.into_any(),
            Level6(seg1, seg2, seg3, seg4, seg5, seg6) => view!{ <Navigate to=format!("{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}") label=label class="pl-18"/> }.into_any(),
            Level7(seg1, seg2, seg3, seg4, seg5, seg6, seg7) => view!{ <Navigate to=format!("{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}/{seg7}") label=label class="pl-21"/> }.into_any(),
            Level8(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8) => view!{ <Navigate to=format!("{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}/{seg7}/{seg8}") label=label class="pl-24"/> }.into_any(),
            Level9(seg1, seg2, seg3, seg4, seg5, seg6, seg7, seg8, seg9) => view!{ <Navigate to=format!("{seg1}/{seg2}/{seg3}/{seg4}/{seg5}/{seg6}/{seg7}/{seg8}/{seg9}") label=label class="pl-27"/> }.into_any(),
        }
    }
}

/// Define routes in the application
#[derive(Debug,Clone)]
pub struct RouteDef {
    /// path part in the url
    pub path: &'static str,
    /// Label in  the menu
    pub label: &'static str,
    /// component to render when the route is matched
    pub component: fn() -> AnyView,
    /// optional children for nested routes
    pub subroutes: &'static [RouteDef],
}

impl RouteDef{
    /// Converts the route to list of routes that can be used in `leptos_router`
    pub fn as_routes(&self, prefix: PathSpec) -> Vec<AnyNestedRoute> {
        let my_path: PathSpec = prefix.extend(self.path);

        let mut routes: Vec<AnyNestedRoute> = vec![
            my_path.as_route(
                self.component
            )
        ];

        routes.extend(
            self.subroutes.
                iter().
                flat_map(|r| {
                    r.as_routes(my_path)
                })
        );

        routes
    }

    /// Builds menu items
    pub fn as_menu_items(&self, prefix: PathSpec) -> Vec<AnyView> {
        let my_path: PathSpec = prefix.extend(self.path);

        let mut views = vec![
            my_path.as_navigation_view(self.label)
        ];
        

        views.extend(
            self.subroutes.
                iter().
                flat_map(|r| {
                    r.as_menu_items(my_path)
                })
        );
        
        views
    }

    /// Creates a new page route with a story
    /// 
    /// This story doesn't have any subseries defined
    pub const fn page<S: 'static + Story + Default + Copy + ThreadSafe>(path: &'static str, label: &'static str) -> RouteDef {
        RouteDef{ 
            path,
            label,
            component: || view!{ <Page<S> /> }.into_any(),
            subroutes: &[]
        }
    }

    /// Creates a new page route with a story and it's related sub-stories
    pub const fn component<S: 'static + Story + Default + Copy + ThreadSafe>(path: &'static str, label: &'static str, subroutes: &'static [RouteDef]) -> RouteDef {
        RouteDef{ 
            path,
            label,
            component: || view!{ <Page<S> /> }.into_any(),
            subroutes
        }
    }

    /// Creates a new section route
    /// 
    /// Section is the part of storybook which encompasses a bigger group of stories
    pub const fn section<S: 'static + Section + Default + Copy + Send>(path: &'static str, label: &'static str, subroutes: &'static [RouteDef]) -> RouteDef {
        RouteDef{
            path,
            label,
            component: || view!{ <section::Section<S> /> }.into_any(),
            subroutes,
        }
    }
}