use crate::codegen::ir::namespace::Namespace;

pub(crate) mod field;
pub(crate) mod ty;

#[derive(Debug)]
pub(crate) struct ApiDartGeneratedClass {
    pub(crate) namespace: Namespace,
    pub(crate) code: String,
}
