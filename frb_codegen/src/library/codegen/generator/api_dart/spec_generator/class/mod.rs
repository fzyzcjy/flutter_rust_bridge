use crate::codegen::ir::namespace::Namespace;
use crate::utils::basic_code::DartBasicHeaderCode;
use serde::Serialize;

pub(crate) mod field;
pub(crate) mod method;
pub(crate) mod ty;
pub(super) mod misc;

#[derive(Debug, Serialize, Default)]
pub(crate) struct ApiDartGeneratedClass {
    pub(crate) header: DartBasicHeaderCode,
    pub(crate) namespace: Namespace,
    pub(crate) code: String,
    pub(crate) needs_freezed: bool,
}
