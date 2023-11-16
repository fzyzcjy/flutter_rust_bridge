use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::DartApiGeneratorClassTrait;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;

impl<'a> DartApiGeneratorClassTrait for DelegateDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        match &self.ir {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                EnumRefDartApiGenerator::new(ir.clone(), self.context.clone()).generate_class()
            }
            IrTypeDelegate::Array(array) => Some(format!(
                "
                class {0} extends NonGrowableListView<{1}> {{
                    static const arraySize = {2};
                    {0}({3} inner)
                        : assert(inner.length == arraySize),
                          super(inner);
                    {0}.unchecked({3} inner)
                        : super(inner);
                    {4}
                  }}
                ",
                array.dart_api_type(self.context.ir_pack),
                DartApiGenerator::new(array.inner(), self.context.ir_pack).dart_api_type(),
                array.length(),
                DartApiGenerator::new(array.get_delegate(), self.context.ir_pack).dart_api_type(),
                array.dart_init_method(),
            )),
            _ => None,
        }
    }
}
