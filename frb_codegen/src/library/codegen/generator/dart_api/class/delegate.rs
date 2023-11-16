use crate::codegen::generator::dart_api::class::DartApiClassGeneratorTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::dart_api_class_generator_struct;

dart_api_class_generator_struct!(DelegateDartApiClassGenerator, IrTypeDelegate);

impl<'a> DartApiClassGeneratorTrait for DelegateDartApiClassGenerator<'a> {
    fn generate(&self) -> String {
        match &self.ir {
            IrTypeDelegate::PrimitiveEnum { ir, .. } => super::TypeEnumRefGenerator {
                ir: ir.clone(),
                context: self.context.clone(),
            }
            .structs(),
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
        }
    }
}
