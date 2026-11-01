//! Tailwind4 breakpoints tracking

use std::borrow::Cow;
use std::fmt::Debug;

use leptos::leptos_dom::logging::console_debug_log;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::Memo;
use leptos::prelude::Signal;
use leptos::prelude::With;
use leptos_use::use_media_query;
use utils::prelude::ThreadSafe;

/// Converts a value to the media query
pub trait AsMediaQuery {
    /// creates a media query
    fn as_media_query(&self) -> Cow<'_, str>;
}

/// Breakpoints defined in tailwind 4
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tailwindcss4Breakpoints{
    /// Tailwindcss 4 `sm` breakpoint
    /// 
    /// By default it's `40rem`. In standard tailwind setup it's `640px`
    /// 
    /// ```css
    /// @media (width >= 40rem) { ... }
    /// ``````
    S,
    /// Tailwindcss 4 `md` breakpoint
    /// 
    /// By default it's `48rem`. In standard tailwind setup it's `768px`
    /// 
    /// ```css
    /// @media (width >= 48rem) { ... }
    /// ```
    M,	
    /// Tailwindcss 4 `lg` breakpoint
    /// 
    /// By default it's `64rem`. In standard tailwind setup it's `1024px`
    /// 
    /// ```css
    /// @media (width >= 64rem) { ... }
    /// ```
    L,
    /// Tailwindcss4 `xl` breakpoint
    /// 
    /// By default it's `80rem`. In standard tailwind setup it's `1280px`
    /// 
    /// ```css
    /// @media (width >= 80rem) { ... }
    /// ```
    XL,
    /// Tailwind 4 `2xl` breakpoint
    /// 
    /// By default it's `96rem`. In standard tailwind setup it's `1536px`
    /// 
    /// ```css
    /// @media (width >= 96rem) { ... }
    /// ```
    #[allow(clippy::upper_case_acronyms)]
    XXL,
}

impl AsMediaQuery for Tailwindcss4Breakpoints {
    fn as_media_query(&self) -> Cow<'_, str> {
        use Tailwindcss4Breakpoints::*;
        match self {
            S => Cow::Borrowed("(min-width: 40rem)"),
            M => Cow::Borrowed("(min-width: 48rem)"),
            L => Cow::Borrowed("(min-width: 64rem)"),
            XL => Cow::Borrowed("(min-width: 80rem)"),
            XXL => Cow::Borrowed("(min-width: 96rem)"),
        }
    }
}

/// Configuration of the breakpoints to be tracked
pub struct UseBreakpointConfiguration<T> {
    /// Map which maps the media query and breakpoints
    breakpoints: Vec<T>,
}

impl UseBreakpointConfiguration<Tailwindcss4Breakpoints> {
    /// Creates configuration with tailwindcss4 breakpoints
    /// 
    /// This configuration differs from `use_leptos` one because we are using
    /// the `rem` units for breakpoints instead of px size
    pub fn tailwind4() -> UseBreakpointConfiguration<Tailwindcss4Breakpoints> {
        use Tailwindcss4Breakpoints::*;

        Self { 
            breakpoints:  vec![
                S, M, L, XL, XXL
            ]
        }
    }
}

/// Value of the signal indicating the transition
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BreakpointTransition<T> {
    /// Breakpoint at which we are crossing
    breakpoint: T,
    /// 
    /// If set to `true` that means we are moving away from `breakpoint` into
    /// smaller screen resolution.
    /// 
    /// If set to `false` that means we've just entered given screen resolution
    /// ratio
    away: bool,
}

/// Allows to get the signals needed to monitor the crossing of the breakpoints
/// in the application
pub struct Breakpoints<T> {
    /// Configuration of the breakpoints
    configuration: UseBreakpointConfiguration<T>
}

impl<T> Breakpoints<T> 
where
    T: AsMediaQuery + ThreadSafe + Clone + PartialOrd + Debug
{
    /// Tracks the transition between breakpoints
    pub fn breakpoint(&self, t: impl Into<Signal<T>>) -> Signal<BreakpointTransition<T>> {
        let t = t.into();
        console_log(&format!("setting up breakpoint {t:?}"));
        let query = Signal::derive(move || {
            let t = t.get();
            console_log("Media query signal");
            t.as_media_query().to_string()
        });
        let media = use_media_query(query);

        let memo = Memo::new(move |v: Option<&BreakpointTransition<T>>| {
            console_log("Breakpoint triggered");
            let away = !media.get();
            match v {
                Some(v) => {
                    let t: T = t.get_untracked();
                    console_debug_log(&format!("Old transition: {t:?}\naway: {away}"));
                    BreakpointTransition{ 
                        breakpoint: t, 
                        away,
                    }
                }
                None => {
                    console_debug_log("No past breakpoint transition");
                    BreakpointTransition{
                        breakpoint: t.get_untracked(),
                        away: false,
                    }
                }
            }
        });

        memo.into()
    }
}

/// Returns the signal which will trigger whenever the page crosses the breakpoint
pub fn use_breakpoint<T>(configuration: UseBreakpointConfiguration<T>) -> Breakpoints<T> {
    Breakpoints{ 
        configuration
    }
}
