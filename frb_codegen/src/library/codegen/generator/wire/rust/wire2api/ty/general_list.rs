use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::IrType::{Delegate, Optional};
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;

impl<'a> WireRustGeneratorWire2apiTrait for GeneralListWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
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

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        general_list_impl_wire2api_body()
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

pub(crate) fn general_list_impl_wire2api_body() -> Acc<Option<String>> {
    Acc {
        wasm: Some(WIRE2API_BODY_WASM.to_owned()),
        io: Some(WIRE2API_BODY_IO.to_owned()),
        ..Default::default()
    }
}

const WIRE2API_BODY_IO: &'static str = "
    let vec = unsafe {
        let wrap = support::box_from_leak_ptr(self);
        support::vec_from_leak_ptr(wrap.ptr, wrap.len)
    };
    vec.into_iter().map(Wire2Api::wire2api).collect()
";
const WIRE2API_BODY_WASM: &'static str =
    "self.dyn_into::<JsArray>().unwrap().iter().map(Wire2Api::wire2api).collect()";
