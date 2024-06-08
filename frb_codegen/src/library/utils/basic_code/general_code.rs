use serde::Serialize;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;

#[derive(Clone, Debug, Serialize)]
pub(crate) enum GeneralCode {
    Dart(GeneralDartCode),
    Rust(GeneralRustCode),
}

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct GeneralDartCode {
    pub header: DartHeaderCode,
    pub body: String,
}

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct GeneralRustCode {
    pub body: String,
}
