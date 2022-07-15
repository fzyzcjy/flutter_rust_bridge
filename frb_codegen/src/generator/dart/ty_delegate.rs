use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;
use crate::utils::BlockIndex;

type_dart_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeDartGeneratorTrait for TypeDelegateGenerator<'_> {
    fn api2wire_body(&self, block_index: BlockIndex) -> Option<String> {
        Some(match self.ir {
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
            IrTypeDelegate::StringList => format!(
                "final ans = inner.new_StringList_{}(raw.length);
                for (var i = 0; i < raw.length; i++){{
                    ans.ref.ptr[i] = _api2wire_String(raw[i]);
                }}
                return ans;",
                block_index
            ),
            IrTypeDelegate::PrimitiveEnum { ref repr, .. } => {
                format!("return _api2wire_{}(raw.index);", repr.safe_ident())
            }
        })
    }

    fn wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::String
            | IrTypeDelegate::SyncReturnVecU8
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_wire2api_simple_type_cast(&self.ir.dart_api_type())
            }
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                format!("return {}.values[raw];", ir.dart_api_type())
            }
        }
    }

    fn structs(&self) -> String {
        if let IrTypeDelegate::PrimitiveEnum { ir, .. } = &self.ir {
            super::TypeEnumRefGenerator {
                ir: ir.clone(),
                context: self.context.clone(),
                dart_api_class_name: None,
            }
            .structs()
        } else {
            "".into()
        }
    }
}
