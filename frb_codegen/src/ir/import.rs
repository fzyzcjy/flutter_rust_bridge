#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IrImport {
    pub uri: String,
    pub alias: Option<String>,
}
