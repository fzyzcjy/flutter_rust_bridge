pub(crate) mod consts;

/// Please refer to `TemplateArg` for doc
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Template {
    App,
    Plugin,
}
