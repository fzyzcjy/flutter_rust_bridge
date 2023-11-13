use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;

use super::generate_import;
use super::TypeGeneralListGenerator;

type_rust_generator_struct!(TypeOptionalListGenerator, IrTypeOptionalList);

impl TypeRustGeneratorTrait for TypeOptionalListGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: Some(TypeGeneralListGenerator::WIRE2API_BODY_WASM.to_owned()),
            io: Some(TypeGeneralListGenerator::WIRE2API_BODY_IO.to_owned()),
            ..Default::default()
        }
    }
    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        Some(vec![
            format!(
                "ptr: *mut *mut {}",
                self.ir.inner.rust_wire_type(Target::Io)
            ),
            "len: i32".to_string(),
        ])
    }
    fn allocate_funcs(
        &self,
        collector: &mut super::ExternFuncCollector,
        block_index: crate::utils::misc::BlockIndex,
    ) -> Acc<Option<String>> {
        Acc {
            io: Some(collector.generate(
                &format!("new_{}_{}", self.ir.safe_ident(), block_index),
                [("len: i32", "int")],
                Some(&format!("*mut {}", self.ir.rust_wire_type(Target::Io))),
                &format!(
                    "let wrap = {} {{ ptr: support::new_leak_vec_ptr(core::ptr::null_mut(), len), len }};
                    support::new_leak_box_ptr(wrap)",
                    self.ir.rust_wire_type(Target::Io)
                ),
                Target::Io,
            )),
            ..Default::default()
        }
    }
    fn imports(&self) -> Option<String> {
        generate_import(&self.ir.inner, self.context.ir_file, self.context.config)
    }
}
