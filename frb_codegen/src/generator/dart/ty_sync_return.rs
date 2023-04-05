use delegate::delegate;

use crate::generator::dart::*;
use crate::target::Acc;

use super::TypeDartGeneratorTrait;

#[derive(Debug, Clone)]
pub struct TypeSyncReturnGenerator<'a> {
    inner: Box<TypeDartGenerator<'a>>,
}

impl<'a> TypeSyncReturnGenerator<'a> {
    pub fn new(ir: IrTypeSyncReturn, context: TypeGeneratorContext<'a>) -> Self {
        TypeSyncReturnGenerator {
            inner: Box::new(TypeDartGenerator::new(
                ir.into_inner(),
                context.ir_file,
                context.config,
            )),
        }
    }
}

impl<'a> TypeDartGeneratorTrait for TypeSyncReturnGenerator<'a> {
    delegate! {
        to self.inner {
            fn api2wire_body(&self, shared_dart_api2wire_funcs: &Option<Acc<String>>) -> Acc<Option<String>>;
            fn api_fill_to_wire_body(
                &self,
                shared_dart_api2wire_funcs: &Option<Acc<String>>,
            ) -> Option<String>;
            fn wire2api_body(&self) -> String;
            fn structs(&self) -> String;
            fn get_context(&self) -> &TypeGeneratorContext;

        }
    }
}
