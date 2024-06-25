pub(crate) mod consts;

/// The type of the project template
#[derive(Debug, Copy, Clone)]
pub enum Template {
    /// A Flutter application
    App,
    /// A shareable Flutter project that can be used across multiple Flutter applictions.
    Plugin,
}
