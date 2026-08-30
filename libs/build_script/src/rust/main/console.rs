//! Handles output for the build scripts
//! 
//! # Basic usage
//! 
//! ```toml
//! [build-dependencies]
//! leptos_forge_build_script = "0.6"
//! ```
//! 
//! In your build script:
//! 
//! ```rust,no_run
//! # #[allow(clippy::needless_doctest_main)]
//! use leptos_forge_build_script::console::{
//!   Console,
//!   ConsoleConfiguration,
//! };
//! 
//! 
//! fn main() {
//! 
//!   let console_configuration = ConsoleConfiguration::default();
//!   let console = Console::new("crate_name", &console_configuration);
//! 
//!   console.info(&"This is an info message"); // Prints
//!                                             // info:  [crate_name]  This is an info message
//! 
//!   let x = 3;
//!   console.warn(&format!("The x = {x}"));    // Prints
//!                                             // warning:  [crate_name]  The x = 3
//! 
//! }
//! ```
//! 
//! For printing colorful messages uses [`build_print`] crate.


use std::marker::PhantomData;

use build_print::error;
use build_print::info;
use build_print::warn;

/// How verbose the output should be
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Only print errors
    ERROR,
    /// Print errors and warnings
    WARNING,
    /// Print errors, warnings and info
    INFO,
}

/// Configuration of the [`Console`]
pub struct ConsoleConfiguration {
    /// Wherever we should enable [`build_print`] support
    /// 
    /// Default: `true`
    build_print: bool,
    /// Level at which messages should be printed
    log_level: LogLevel,
}

impl ConsoleConfiguration {
    /// Console will use the `cargo::error=MESSAGE"` and the `cargo::warning=MESSAGE`
    /// to print the output
    /// 
    /// It disables the [`build_print`] support
    pub fn cargo_colors(mut self) -> Self {
        self.build_print = false;
        self
    }

    /// Console will print the messages up to this level only
    pub fn log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = log_level;
        self
    }
}


impl Default for ConsoleConfiguration {
    fn default() -> Self {
        Self { 
            build_print: true,
            log_level: LogLevel::INFO,
        }
    }
}

/// Prints standard cargo warning
fn cargo_warn(str: &str) {
    std::println!("cargo::warning={str}");
}

/// Prints standard cargo error
fn cargo_error(str: &str) {
    std::println!("cargo::error={str}");
}

/// Handles formatted output for the build scripts
pub struct Console<'this>{
    /// List of tags to prepend to every line of the output
    /// 
    /// Tags are shown at the beginning of the line in the square braces:
    /// 
    /// **Example**: 
    /// ```text
    /// `[forge] Starting a build script`
    /// ```
    /// 
    /// In the sample output above the `[forge]` is a tag
    tags: String,
    /// Configuration of the console output
    configuration: &'this ConsoleConfiguration,
    #[doc(hidden)]
    _this: PhantomData<&'this Self>,
}

impl<'this> Console<'this> {
    /// Create a new instance of the [`Console`]
    /// 
    /// # Arguments
    /// 
    /// - **package_tag** - Human readable name identifying the package so from
    ///   which the output comes from
    pub fn new(package_tag: &'static str, configuration: &'this ConsoleConfiguration) -> Self {
        Self{ 
            tags: format!("[{package_tag}]"),
            configuration,
            _this: PhantomData
        }
    }

    /// Create a new scoped instance of [`Console`] with a new tag
    pub fn stage<'new>(&self, tag: &'static str) -> Console<'new> 
    where
        'this: 'new
    {
        let tags = format!("{}[{tag}]", self.tags);

        Self {
            tags,
            configuration: self.configuration,
            _this: PhantomData
        }
    }

    /// Simple helper function to print the multiline error message to the cargo output
    pub fn error<S: ToString>(&self, s: &S) {
        if self.configuration.log_level < LogLevel::ERROR {
            return;
        }

        for line in s.to_string().split("\n") {
            if self.configuration.build_print {
                error!("{} {line}", self.tags);
            }
            else {
                cargo_error(&format!("  error: {} {line}", self.tags));
            }
        }
    }

    /// Simple helper function to print the multiline warning message to the cargo output
    pub fn warn<S: ToString>(&self, s: &S) {

        if self.configuration.log_level < LogLevel::WARNING {
            return;
        }

        for line in s.to_string().split("\n") {
            if self.configuration.build_print {
                warn!("{}  {line}", self.tags);
            }
            else {
                cargo_warn(&format!("  warning: {} {line}", self.tags));
            }
        }
    }

    /// Simple helper function to print the multiline information message to the cargo output
    pub fn info<S: ToString>(&self, s: &S) {
        if self.configuration.log_level < LogLevel::INFO {
            return;
        }

        for line in s.to_string().split("\n") {
            if self.configuration.build_print {
                info!("{}  {line}", self.tags);
            }
            else {
                cargo_warn(&format!("  info: {} {line}", self.tags));
            }
        }
    }

    /// Simple helper function to print a normal multiline text to the cargo output
    /// 
    /// It's never suppressed by the log level
    pub fn println<S: ToString>(&self, s: &S) {
        for line in s.to_string().split("\n") {
            if self.configuration.build_print {
                println!("  out:  {} {line}", self.tags);
            }
            else {
                cargo_warn(&format!("  out:  {} {line}", self.tags));
            }
        }
    }

}

