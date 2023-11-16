use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::DartApiClassGeneratorTrait;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};

impl<'a> DartApiClassGeneratorTrait for DelegateDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        Some(match &self.ir {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                EnumRefDartApiGenerator::new(ir.clone(), self.context.ir_pack).generate_class()
            }
            IrTypeDelegate::Array(array) => {
                format!(
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
                    array.dart_api_type(),
                    array.inner_dart_api_type(),
                    array.length(),
                    array.get_delegate().dart_api_type(),
                    array.dart_init_method(),
                )
            }
            _ => "".into(),
        })
    }
}
