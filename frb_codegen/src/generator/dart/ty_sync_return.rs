use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::type_dart_generator_struct;
use crate::utils::BlockIndex;

type_dart_generator_struct!(TypeSyncReturnGenerator, IrTypeSyncReturn);

impl TypeDartGeneratorTrait for TypeSyncReturnGenerator<'_> {
    fn api2wire_body(&self, _: BlockIndex) -> Option<String> {
        Some("/*unsupported*/".to_string())
    }

    fn wire2api_body(&self) -> String {
        match self.ir {
            IrTypeSyncReturn::Primitive(ref primitive) => match primitive {
                IrTypePrimitive::Bool => "final dataView = ByteData.view(raw.buffer);
                        return dataView.getUint8(0) != 0;"
                    .into(),
                primitive => {
                    let primitive_name = match primitive {
                        IrTypePrimitive::U8 => "Uint8",
                        IrTypePrimitive::U16 => "Uint16",
                        IrTypePrimitive::U32 => "Uint32",
                        IrTypePrimitive::U64 => "Uint64",
                        IrTypePrimitive::I8 => "Int8",
                        IrTypePrimitive::I16 => "Int16",
                        IrTypePrimitive::I32 => "Int32",
                        IrTypePrimitive::I64 => "Int64",
                        IrTypePrimitive::F32 => "Float32",
                        IrTypePrimitive::F64 => "Float64",
                        _ => "/*unsupported*/",
                    };
                    format!(
                        "{}{}{}",
                        "final dataView = ByteData.view(raw.buffer);
                        return dataView.get",
                        primitive_name,
                        "(0);"
                    )
                }
            },
            IrTypeSyncReturn::String => "return utf8.decode(raw);".into(),
            IrTypeSyncReturn::VecU8 => "return raw;".into(),
        }
    }
}
