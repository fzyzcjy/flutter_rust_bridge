use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::IrType;
use serde::Deserialize;
use crate::codegen::ir::default::IrDefaultValue;

crate::ir! {
pub struct IrField {
    pub ty: IrType,
    pub name: IrIdent,
    pub is_final: bool,
    pub comments: Vec<IrComment>,
    pub default: Option<IrDefaultValue>,

    pub settings: IrFieldSettings,
}

#[derive(Deserialize, Default)]
pub struct IrFieldSettings {
    pub is_in_mirrored_enum: bool,
}
}
