use crate::codegen::ir::namespace::Namespace;
use serde::Serialize;

pub(crate) mod field;
pub(crate) mod ty;

#[derive(Debug, Serialize)]
pub(crate) struct ApiDartGeneratedClass {
    pub(crate) namespace: Namespace,
    pub(crate) code: String,
}
