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
        let not_opaque_body = |ir: &IrTypeSyncReturn| match ir {
            IrTypeSyncReturn::Primitive(ref primitive) => match primitive {
                IrTypePrimitive::Bool => "return uint8ListToBool(raw);".into(),
                // todo
                IrTypePrimitive::Usize => r#"
                    final dataView = ByteData.view(raw.buffer);
                    switch (raw.length) {
                        case 8: {
                          return dataView.getUint64(0);
                        }
                        case 4: {
                          return dataView.getUint32(0);
                        }
                        default: {throw "Unknow lenght pointer.";}
                      }"#
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
                        IrTypePrimitive::Unit => {
                            return format!(
                                r#"
                                return;
                                "#,
                            );
                        }
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
                format!(
                    "var pointBitLen = raw.length ~/ 3;
                var ptrList = List.filled(pointBitLen, 0);
                var dropList = List.filled(pointBitLen, 0);
                var lendList = List.filled(pointBitLen, 0);
                
                List.copyRange(ptrList, 0, raw, 0, pointBitLen);
                List.copyRange(dropList, 0, raw, pointBitLen, pointBitLen*2);
                List.copyRange(lendList, 0, raw, pointBitLen*2);

                int ptr = 0;
                int drop = 0;
                int lend = 0;
                
                if (pointBitLen == 8) {{
                  ptr = ByteData.view(Uint8List.fromList(ptrList).buffer).getUint64(0);
                  drop = ByteData.view(Uint8List.fromList(dropList).buffer).getUint64(0);
                  lend = ByteData.view(Uint8List.fromList(lendList).buffer).getUint64(0);
                }} else if (pointBitLen == 4) {{
                  ptr = ByteData.view(Uint8List.fromList(ptrList).buffer).getUint32(0);
                  drop = ByteData.view(Uint8List.fromList(dropList).buffer).getUint32(0);
                  lend = ByteData.view(Uint8List.fromList(lendList).buffer).getUint32(0);
                }}
            
                return {}.fromRaw(ptr, drop, lend);",
                    o.inner_dart
                )
            }
            IrTypeSyncReturn::Option(_) => "".into(),
        };
        if let IrTypeSyncReturn::Option(ty) = &self.ir {
            if let IrTypeSyncReturn::Option(_) = **ty {
                panic!("Nested option is not suppored.")
            }
            format!("if (raw == null) return null; {}", not_opaque_body(ty))
        } else {
            not_opaque_body(&self.ir)
        }
    }
}
