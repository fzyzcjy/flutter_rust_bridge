use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegatePrimitiveEnum,
};
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;

impl<'a> DartApiGeneratorClassTrait for DelegateDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        match &self.ir {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                EnumRefDartApiGenerator::new(ir.clone(), self.context.clone()).generate_class()
            }
            IrTypeDelegate::Array(array) => {
                let self_dart_api_type = array.dart_api_type(self.context.clone());
                let inner_dart_api_type =
                    DartApiGenerator::new(array.inner(), self.context.clone()).dart_api_type();
                let delegate_dart_api_type =
                    DartApiGenerator::new(array.get_delegate(), self.context.clone())
                        .dart_api_type();

                let array_length = array.length();

                let dart_init_method = match array {
                    IrTypeDelegateArray::GeneralArray { .. } => format!(
                        "{self_dart_api_type}.init({inner_dart_api_type} fill): super(List<{inner_dart_api_type}>.filled(arraySize,fill));",
                    ),
                    IrTypeDelegateArray::PrimitiveArray { .. } => format!(
                        "{self_dart_api_type}.init(): super({delegate_dart_api_type}(arraySize));",
                    ),
                };

                Some(format!(
                    "
                class {self_dart_api_type} extends NonGrowableListView<{inner_dart_api_type}> {{
                    static const arraySize = {array_length};
                    {self_dart_api_type}({delegate_dart_api_type} inner)
                        : assert(inner.length == arraySize),
                          super(inner);
                    {self_dart_api_type}.unchecked({delegate_dart_api_type} inner)
                        : super(inner);
                    {dart_init_method}
                  }}
                "
                ))
            }
            _ => None,
        }
    }
}
