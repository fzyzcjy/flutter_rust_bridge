use delegate_attr::delegate;

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
                context.config,
                context.all_configs,
            )),
        }
    }
}

#[delegate(self.inner)]
impl<'a> TypeDartGeneratorTrait for TypeSyncReturnGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {}
    fn api_fill_to_wire_body(&self) -> Option<String> {}
    fn wire2api_body(&self) -> String {}
    fn structs(&self) -> String {}
}
