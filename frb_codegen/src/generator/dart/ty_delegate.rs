use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeDelegateGenerator(IrTypeDelegate);

impl TypeDartGeneratorTrait for TypeDelegateGenerator {
    fn api2wire_body(&self) -> String {
        match self.0 {
            IrTypeDelegate::String => {
                "return _api2wire_uint_8_list(utf8.encoder.convert(raw));".to_string()
            }
            IrTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".to_string(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                format!("return _api2wire_{}(raw);", d.get_delegate().safe_ident())
            }
            IrTypeDelegate::StringList => "final ans = inner.new_StringList(raw.length);
            for (var i = 0; i < raw.length; i++) {
                ans.ref.ptr[i] = _api2wire_String(raw[i]);
            }
            return ans;"
                .to_owned(),
        }
    }

    fn api_fill_to_wire_body(&self) -> String {
        match self.0 {
            IrTypeDelegate::String
            | IrTypeDelegate::SyncReturnVecU8
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_wire2api_simple_type_cast(&d.dart_api_type())
            }
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
        }
    }
}
