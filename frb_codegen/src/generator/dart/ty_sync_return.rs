use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeSyncReturnGenerator, IrTypeSyncReturn);

impl TypeDartGeneratorTrait for TypeSyncReturnGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        unimplemented!("SyncReturn generator for Dart: api2wire_body is not supported")
    }

    fn wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeSyncReturn::Primitive(ref primitive) => match primitive {
                IrTypePrimitive::Bool => "return uint8ListToBool(raw);".into(),
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
                        _ => panic!(
                            "SyncReturn generator for Dart: type {} is not supported",
                            primitive.rust_api_type()
                        ),
                    };
                    format!(
                        r#"
                        final dataView = ByteData.view(raw.buffer);
                        return dataView.get{primitive_name}(0);
                        "#,
                        primitive_name = primitive_name
                    )
                }
            },
            IrTypeSyncReturn::String => "return utf8.decode(raw);".into(),
            IrTypeSyncReturn::VecU8 => "return raw;".into(),
            IrTypeSyncReturn::Opaque(o) => {
                format!("var intLen = raw[0] ~/ 8;
                var ptrList = List.filled(intLen, 0);
                var dropList = List.filled(intLen, 0);
                var lendList = List.filled(intLen, 0);
                var j = 0;
                for (var i = 1; i < 1 + intLen; ++i, ++j) {{
                  ptrList[j] = raw[i];
                }}
                j = 0;
                for (var i = 1 + intLen; i < 1 + intLen * 2; ++i, ++j) {{
                  dropList[j] = raw[i];
                }}
                j = 0;
                for (var i = 1 + intLen * 2; i < 1 + intLen * 3; ++i, ++j) {{
                  lendList[j] = raw[i];
                }}
            
                var a = ByteData.view(Uint8List.fromList(ptrList).buffer, 0, 8).getUint64(0);
                var b = ByteData.view(Uint8List.fromList(dropList).buffer, 0, 8).getUint64(0);
                var c = ByteData.view(Uint8List.fromList(lendList).buffer, 0, 8).getUint64(0);
            
                return {}.fromRaw(a, b, c);", o.inner_dart)
            },
        }
    }
}
