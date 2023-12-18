use crate::generator::rust::ty::*;
use crate::generator::rust::{generate_import, generate_list_allocate_func, ExternFuncCollector};
use crate::ir::*;
use crate::target::{Acc, Target};
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeGeneralListGenerator, IrTypeGeneralList);

impl TypeGeneralListGenerator<'_> {
    pub const WIRE2API_BODY_IO: &'static str = "
            let vec = unsafe {
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            };
            vec.into_iter().map(Wire2Api::wire2api).collect()";
    pub const WIRE2API_BODY_WASM: &'static str =
        "self.dyn_into::<JsArray>().unwrap().iter().map(Wire2Api::wire2api).collect()";
}

impl TypeRustGeneratorTrait for TypeGeneralListGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        let prefix = self.get_wire2api_prefix(&self.ir.inner);
        // Replace prefix to WIRE2API_BODY_IO and WIRE2API_BODY_WASM
        let wire2api_body_io =
            TypeGeneralListGenerator::WIRE2API_BODY_IO.replace("Wire2Api", &prefix);
        let wire2api_body_wasm =
            TypeGeneralListGenerator::WIRE2API_BODY_WASM.replace("Wire2Api", &prefix);
        Acc {
            io: Some(wire2api_body_io),
            wasm: Some(wire2api_body_wasm),
            ..Default::default()
        }
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        Some(vec![
            format!(
                "ptr: *mut {}{}",
                self.ir.inner.rust_ptr_modifier(),
                self.ir.inner.rust_wire_type(Target::Io)
            ),
            "len: i32".to_string(),
        ])
    }

    fn allocate_funcs(&self, collector: &mut ExternFuncCollector) -> Acc<Option<String>> {
        Acc {
            io: Some(generate_list_allocate_func(
                collector,
                &self.ir.safe_ident(),
                &self.ir,
                &self.ir.inner,
            )),
            ..Default::default()
        }
    }

    fn imports(&self) -> Option<String> {
        generate_import(
            &self.ir.inner,
            self.context.config,
            self.context.all_configs,
        )
    }
}
