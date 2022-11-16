use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeDartOpaqueGenerator, IrTypeDartOpaque);

impl TypeRustGeneratorTrait for TypeDartOpaqueGenerator<'_> {
    fn wire2api_body(&self) -> crate::target::Acc<Option<String>> {
        Acc {
            io: Some(
                "DartOpaque::new(self)"
                .into(),
            ),
            ..Default::default()
        }
    }

    /// Handles JsValue to Self conversions.
    fn wire2api_jsvalue(&self) -> Option<Cow<str>> {None}

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
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

    fn wrap_obj(&self, _obj: String) -> String {
        "".to_owned()
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


    fn imports(&self) -> Option<String> {
        None
    }
}