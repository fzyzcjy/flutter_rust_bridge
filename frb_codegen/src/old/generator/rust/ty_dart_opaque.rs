use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

use super::{ExternFuncCollector, NO_PARAMS};

type_rust_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeRustGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn generate_impl_wire2api_body(&self) -> crate::target::Acc<Option<String>> {
        Acc {
            io: Some("unsafe{DartOpaque::new(self.handle as _, self.port)}".to_owned()),
            wasm: Some(
                "let arr = self.dyn_into::<JsArray>().unwrap();
                unsafe{DartOpaque::new(arr.get(0), arr.get(1))}"
                    .to_owned(),
            ),
            ..Default::default()
        }
    }

    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        _block_index: BlockIndex,
    ) -> Acc<Option<String>> {
        let rust_wire = self.ir.rust_wire_type(crate::target::Target::Io);

        Acc {
            io: Some(collector.generate(
                &format!("new_{}", self.ir.safe_ident()),
                NO_PARAMS,
                Some(&format!(
                    "{}{}",
                    self.ir.rust_wire_modifier(crate::target::Target::Io),
                    rust_wire
                )),
                &format!("{rust_wire}::new_with_null_ptr()"),
                crate::target::Target::Io,
            )),
            ..Default::default()
        }
    }

    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        format!(
            "impl NewWithNullPtr for {} {{
                fn new_with_null_ptr() -> Self {{
                    Self {{ port: 0, handle: 0 }}
                }}
            }}",
            self.ir.rust_wire_type(crate::target::Target::Io)
        )
    }
}
