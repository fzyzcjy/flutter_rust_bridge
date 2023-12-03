use crate::codegen::ir::namespace::Namespace;
use crate::utils::basic_code::DartBasicHeaderCode;
use anyhow::ensure;
use serde::Serialize;

pub(crate) mod field;
pub(crate) mod ty;

#[derive(Debug, Serialize, Default)]
pub(crate) struct ApiDartGeneratedClass {
    pub(crate) header: DartBasicHeaderCode,
    pub(crate) namespace: Namespace,
    pub(crate) class_name: String,
    pub(crate) code: String,
    pub(crate) needs_freezed: bool,
}

impl ApiDartGeneratedClass {
    pub(crate) fn sanity_check(&self) -> anyhow::Result<()> {
        let class_name = &self.class_name;
        ensure!(
            self.code.contains(&format!("class {class_name}"))
                || self.code.contains(&format!("enum {class_name}"))
        );
        Ok(())
    }
}
