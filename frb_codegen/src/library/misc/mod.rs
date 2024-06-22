pub(crate) mod consts;

/// The type of the project
pub enum ProjectType {
    /// A Flutter application
    App,
    /// A shareable Flutter project containing an API in Dart code with 
    /// platform-specific implementations
    Plugin,
}