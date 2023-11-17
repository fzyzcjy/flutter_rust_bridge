use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustClassGeneratorClassTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::IrType::{Delegate, Optional};
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;

impl<'a> WireRustClassGeneratorClassTrait for GeneralListWireRustGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            &self.context,
            &vec![
                format!(
                    "ptr: *mut {}{}",
                    general_list_maybe_extra_pointer_indirection(&self.ir),
                    WireRustGenerator::new(*self.ir.inner.clone(), self.context.clone())
                        .rust_wire_type(Target::Io)
                ),
                "len: i32".to_string(),
            ],
        ))
    }
}

/// Does it need additional indirection for types put behind a vector
pub(crate) fn general_list_maybe_extra_pointer_indirection(ir: &IrTypeGeneralList) -> &'static str {
    if matches!(*ir.inner, Optional(_) | Delegate(IrTypeDelegate::String)) {
        "*mut "
    } else {
        ""
    }
}
