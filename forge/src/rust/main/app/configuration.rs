//! Module contains the configuration setting for the leptos_forge application
//! 
use reactive_stores::PatchField;
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
    /// Configuration of the logo
    pub logo: LogoConfiguration,
    /// Configuration of the test runner
    pub tests_runner: TestRunnerConfiguration,
    /// Canvas configuration
    pub canvas: CanvasConfiguration,
    /// Menu configuration
    pub menu: MenuConfiguration,
    /// control panel configuration
    pub control_panel: ControlPanelConfiguration,
    /// documentation panel configuration
    /// 
    /// Documentation panel also contains a test runner facilities
    pub documentation_panel: DocumentationPanelConfiguration,
}

impl Into<Store<LeptosForgeConfiguration>> for LeptosForgeConfiguration {
    fn into(self) -> Store<LeptosForgeConfiguration> {
        Store::new(self)
    }
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

/// Configuration of the canvas
#[derive(Debug, Clone, Copy, Store, Patch)]
pub struct CanvasConfiguration {
    /// Decides wherever background of the canvas is light or dark checkers pattern
    /// 
    /// - **`None`** - system theme dependant, default value
    /// - **[`Some(ForgeTheme::Light)`][ForgeTheme::Light]** - use light themed checkers
    /// - **[`Some(ForgeTheme::Dark)`][ForgeTheme::Dark]** - use dark themed checkers
    pub background: Option<ForgeTheme>,
}

impl Default for CanvasConfiguration {
    fn default() -> Self {
        Self{ 
            background: None 
        }
    }
}

/// Enumeration of possible themes
#[derive(Debug, Clone, Copy, Store, PartialEq)]
pub enum ForgeTheme {
    /// Use light theme
    Light,
    /// Use dark theme
    Dark
}

impl PatchField for ForgeTheme {
    fn patch_field(
        &mut self,
        new: Self,
        path: &reactive_stores::StorePath,
        notify: &mut dyn FnMut(&reactive_stores::StorePath),
        _keys: Option<&reactive_stores::KeyMap>,
    ) {
        if *self != new {
            *self = new;
            notify(path);
        }
    }
}

/// Configuration of the menu bar
#[derive(Debug, Clone, Copy, Store, Patch)]
pub struct MenuConfiguration {
    /// Controls visibility of the menu bar
    /// 
    /// # Default value
    /// 
    /// By default the menu bar is visible (this value is set to `true`).
    /// 
    /// # UX consideration
    /// 
    /// Setting it to `false` will remove the menu bar from the ui. If you
    /// decide to change this value you should provide some means other
    /// then reload to bring the menu bar back
    visible: bool,
}

impl Default for MenuConfiguration {
    fn default() -> Self {
        Self {
            visible: true,
        }
    }
}

/// Configuration of the control panel
/// 
/// Control panel is visible only when showing the stories directly
#[derive(Debug, Clone, Copy, Store, Patch)]
pub struct ControlPanelConfiguration {
    /// Controls visibility of the control panel
    /// 
    /// # Default value
    /// 
    /// By default the control panel is visible (this value is set to `true`).
    /// 
    /// # UX consideration
    /// 
    /// Setting it to `false` will remove the control panel from the ui. If you
    /// decide to change this value you should provide some means other
    /// then reload to revert the control panel back
    visible: bool,
}

impl Default for ControlPanelConfiguration {
    fn default() -> Self {
        Self {
            visible: true,
        }
    }
}

/// Configuration of the documentation panel
/// 
/// Documentation panel is visible only when showing the stories directly
#[derive(Debug, Clone, Copy, Store, Patch)]
pub struct DocumentationPanelConfiguration {
    /// Controls visibility of the documentation panel
    /// 
    /// # Default value
    /// 
    /// By default the documentation panel is visible (this value is set to `true`).
    /// 
    /// # UX consideration
    /// 
    /// Setting it to `false` will remove the documentation panel from the ui. If you
    /// decide to change this value you should provide some means other
    /// then reload to revert the documentation panel back
    visible: bool,
}

impl Default for DocumentationPanelConfiguration {
    fn default() -> Self {
        Self {
            visible: true,
        }
    }
}