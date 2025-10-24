//! Handles formatted output for the build scripts
//! 

use std::marker::PhantomData;

use build_print::error;
use build_print::info;
use build_print::warn;

/// Configuration of the [`Console`]
pub struct ConsoleConfiguration {
    /// Wherever we should enable [`build_print`] support
    /// 
    /// Default: `true`
    build_print: bool,
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
}


impl Default for ConsoleConfiguration {
    fn default() -> Self {
        Self { build_print: true }
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

