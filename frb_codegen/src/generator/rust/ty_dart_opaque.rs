use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::BlockIndex;

use super::{ExternFuncCollector, NO_PARAMS};

type_rust_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeRustGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn wire2api_body(&self) -> crate::target::Acc<Option<String>> {
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
                &format!("{}::new_with_null_ptr()", rust_wire,),
                crate::target::Target::Io,
            )),
            ..Default::default()
        }
    }

    /// Handles JsValue to Self conversions.
    fn wire2api_jsvalue(&self) -> Option<Cow<str>> {
        None
    }

    fn static_checks(&self) -> Option<String> {
        None
    }

    fn wrapper_struct(&self) -> Option<String> {
        None
    }

    fn self_access(&self, _obj: String) -> String {
        "".to_owned()
    }

    fn wrap_obj(&self, obj: String, _wired_fallible_func: bool) -> String {
        obj
    }

    fn convert_to_dart(&self, obj: String) -> String {
        format!("{}.into_dart()", obj)
    }

    fn structs(&self) -> String {
        "".to_owned()
    }

    fn impl_intodart(&self) -> String {
        "".to_owned()
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        Some(vec!["port: i64".to_owned(), "handle: usize".to_owned()])
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

    fn imports(&self) -> Option<String> {
        None
    }
}
