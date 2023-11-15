use std::path::PathBuf;
use derivative::Derivative;
use syn::Visibility;

#[derive(Clone)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Module {
    pub visibility: Visibility,
    pub file_path: PathBuf,
    pub module_path: Vec<String>,
    #[derivative(Debug="ignore")]
    pub source: Option<ModuleSource>,
    pub scope: Option<ModuleScope>,
}
