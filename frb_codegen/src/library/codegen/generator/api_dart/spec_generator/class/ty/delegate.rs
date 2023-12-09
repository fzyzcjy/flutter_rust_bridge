use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegateArrayMode, IrTypeDelegatePrimitiveEnum,
};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::utils::basic_code::DartBasicHeaderCode;

impl<'a> ApiDartGeneratorClassTrait for DelegateApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        match &self.ir {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                EnumRefApiDartGenerator::new(ir.clone(), self.context).generate_class()
            }
            IrTypeDelegate::Array(array) => generate_array(array, self.context),
            _ => None,
        }
    }
}

fn generate_array(
    array: &IrTypeDelegateArray,
    context: ApiDartGeneratorContext,
) -> Option<ApiDartGeneratedClass> {
    let self_dart_api_type = array.dart_api_type(context);
    let inner_dart_api_type = ApiDartGenerator::new(array.inner(), context).dart_api_type();
    let delegate_dart_api_type =
        ApiDartGenerator::new(array.get_delegate(), context).dart_api_type();

    let array_length = array.length;

    let dart_init_method = match array.mode {
            IrTypeDelegateArrayMode::General(..) => format!(
                "{self_dart_api_type}.init({inner_dart_api_type} fill): super(List<{inner_dart_api_type}>.filled(arraySize,fill));",
            ),
            IrTypeDelegateArrayMode::Primitive(..) => format!(
                "{self_dart_api_type}.init(): super({delegate_dart_api_type}(arraySize));",
            ),
        };

    Some(ApiDartGeneratedClass {
        header: DartBasicHeaderCode {
            import: "import 'package:collection/collection.dart';\n".to_owned(),
            ..Default::default()
        },
        namespace: array.namespace.clone(),
        code: format!(
            "
            class {self_dart_api_type} extends NonGrowableListView<{inner_dart_api_type}> {{
                static const arraySize = {array_length};

                @internal
                {delegate_dart_api_type} get inner => _inner;
                final {delegate_dart_api_type} _inner;

                {self_dart_api_type}(this._inner)
                    : assert(_inner.length == arraySize),
                      super(_inner);
  
                {self_dart_api_type}.init() : this({delegate_dart_api_type}(arraySize));

                {dart_init_method}
              }}
            "
        ),
        needs_freezed: false,
    })
}
