//! Module contains the configuration setting for the leptos_forge application
//! 
use reactive_stores::Store;
use reactive_stores::Patch;

/// Container for the whole `leptos_forge` configuration options
/// 
/// Configuration options are divided into the two parts
/// 
/// 1. Visual part of the application
///    
///    This includes things like logo, it's alternative text, etc...
/// 
/// 2. Functional part of the application
/// 
///    Currently only test runner configuration
#[derive(Debug, Store, Patch, Default)]
pub struct LeptosForgeConfiguration {
    /// Configuration of visual part of the leptos_forge application
    pub visuals: VisualConfiguration,
    /// Configuration of the functional part of the leptos_forge application
    pub functional: FunctionalConfiguration
}

/// Describes the visual aspect of the leptos_forge application
#[derive(Debug, Store, Patch, Default)]
pub struct VisualConfiguration {
    /// Configuration of the logo at the top of the menu
    pub logo: LogoConfiguration
}

/// Configuration of the logo at the top of the menu
/// 
/// # Default value
/// 
/// Default doesn't show the logo at the top of the menu
#[derive(Debug, Store, Patch, Default)]
pub struct LogoConfiguration {
    /// Path to the logo image
    /// 
    /// If set to `None` it will not show any logo at the top of the menu
    pub path: Option<String>,
    /// Alternative text
    pub alt: Option<String>
}

/// Configuration of the functional parts of the leptos_forge application
#[derive(Debug, Store, Patch, Default)]
pub struct FunctionalConfiguration {
    /// Configuration of the test runner
    pub tests_runner: TestRunnerConfiguration
}

/// Configuration of the test runner
#[derive(Debug, Store, Patch)]
pub struct TestRunnerConfiguration {
    /// Minimum time the test runner will wait between applying the step and 
    /// running a validation
    ///
    /// On one hand side you should keep this value as low as possible, so your
    /// tests are run fast. On the other hand you must give your tests enought
    /// time so the browser will apply the changes to the html before testing
    /// the results of a step
    pub validation_delay: u64,
    /// Delay between steps
    /// 
    /// Minimum time the test runner will wait between finishing the validation
    /// step and running the next step
    /// 
    /// You should keep it as small as possible so your tests are run fast. On the
    /// other hand it should be slow enough so you can understand what happens
    pub step_delay: u64,
}

impl Default for TestRunnerConfiguration {
    fn default() -> Self {
        Self {
            step_delay: 250,
            validation_delay: 50,
        }
    }
}