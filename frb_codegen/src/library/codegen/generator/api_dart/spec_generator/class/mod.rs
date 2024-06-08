use crate::utils::basic_code::dart_basic_header_code::DartBasicHeaderCode;
use crate::utils::namespace::Namespace;
use serde::Serialize;

pub(crate) mod field;
pub(crate) mod method;
pub(super) mod misc;
pub(crate) mod ty;

#[derive(Debug, Serialize)]
pub(crate) struct ApiDartGeneratedClass {
    pub(crate) header: DartBasicHeaderCode,
    pub(crate) namespace: Namespace,
    pub(crate) class_name: String,
    pub(crate) code: String,
    pub(crate) needs_freezed: bool,
}
