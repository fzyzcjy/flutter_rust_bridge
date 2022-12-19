use std::borrow::Cow;
use std::fmt::Debug;

use delegate::delegate;

use crate::generator::rust::*;
use crate::target::Acc;

use super::TypeRustGeneratorTrait;

#[derive(Debug, Clone)]
pub struct TypeSyncReturnGenerator<'a> {
    inner: Box<TypeRustGenerator<'a>>,
}

impl<'a> TypeSyncReturnGenerator<'a> {
    pub fn new(ir: IrTypeSyncReturn, context: TypeGeneratorContext<'a>) -> Self {
        TypeSyncReturnGenerator {
            inner: Box::new(TypeRustGenerator::new(
                ir.into_inner(),
                context.ir_file,
                context.config,
            )),
        }
    }
}

impl<'a> TypeRustGeneratorTrait for TypeSyncReturnGenerator<'a> {
    delegate! {
        to self.inner {
            fn wire2api_body(&self) -> Acc<Option<String>>;
            fn wire2api_jsvalue(&self) -> Option<Cow<str>>;
            fn wire_struct_fields(&self) -> Option<Vec<String>>;
            fn static_checks(&self) -> Option<String>;
            fn wrapper_struct(&self) -> Option<String>;
            fn self_access(&self, obj: String) -> String;
            fn wrap_obj(&self, obj: String, _wired_fallible_func: bool) -> String;
            fn convert_to_dart(&self, obj: String) -> String;
            fn structs(&self) -> String;
            fn allocate_funcs(
                &self,
                _collector: &mut ExternFuncCollector,
                _block_index: BlockIndex,
            ) -> Acc<Option<String>>;
            fn related_funcs(
                &self,
                _collector: &mut ExternFuncCollector,
                _block_index: BlockIndex,
            ) -> Acc<Option<String>>;
            fn impl_intodart(&self) -> String;
            fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String;
            fn imports(&self) -> Option<String>;
        }
    }
}
