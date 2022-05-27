use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeDartGeneratorTrait for TypeDelegateGenerator<'_> {
    fn api2wire_body(&self) -> Option<String> {
        Some(match &self.ir {
            IrTypeDelegate::String => {
                "return _api2wire_uint_8_list(utf8.encoder.convert(raw));".to_string()
            }
            IrTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".to_string(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                format!(
                    "return _api2wire_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                )
            }
            IrTypeDelegate::StringList => "final ans = inner.new_StringList(raw.length);
            for (var i = 0; i < raw.length; i++) {
                ans.ref.ptr[i] = _api2wire_String(raw[i]);
            }
            return ans;"
                .to_owned(),
            IrTypeDelegate::ArrayPrimitive {
                primitive: _,
                len: _,
            } => format!(
                "final ans = inner.new_{}(raw.length);
                ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
                return ans;",
                self.ir.get_delegate().safe_ident(),
            ),
            IrTypeDelegate::ArrayGeneral {
                ir_type_general_list,
                len: _,
            } => format!(
                "final ans = inner.new_{}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;",
                ir_type_general_list.safe_ident(),
                ir_type_general_list.inner.safe_ident()
            ),
        })
    }

    fn wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::String
            | IrTypeDelegate::SyncReturnVecU8
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_)
            | IrTypeDelegate::ArrayPrimitive {
                primitive: _,
                len: _,
            } => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
            IrTypeDelegate::ArrayGeneral {
                ir_type_general_list,
                len: _,
            } => {
                format!(
                    "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
                    ir_type_general_list.inner.safe_ident()
                )
            }
        }
    }
}
