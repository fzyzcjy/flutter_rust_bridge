use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeDartGeneratorTrait for TypeDelegateGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        match self.ir {
            IrTypeDelegate::Array(ref array) => match array {
                IrTypeDelegateArray::GeneralArray { .. } => Acc::distribute(Some(format!(
                    "return api2wire_{}(raw);",
                    array.get_delegate().safe_ident(),
                ))),
                IrTypeDelegateArray::PrimitiveArray { length, .. } => Acc {
                    io: Some(format!(
                        "final ans = inner.new_{}_{}({length});
                                    ans.ref.ptr.asTypedList({length}).setAll(0, raw);
                                    return ans;",
                        array.get_delegate().safe_ident(),
                        self.context.config.block_index,
                    )),
                    wasm: Some(format!(
                        "return {}.fromList(raw);",
                        array.get_delegate().dart_api_type()
                    )),
                    ..Default::default()
                },
            },

            IrTypeDelegate::String => Acc {
                io: Some("return api2wire_uint_8_list(utf8.encoder.convert(raw));".into()),
                wasm: Some("return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                // In this case, even though the body is the same, their types are different
                // and must be split.
                let body = format!(
                    "return api2wire_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                );
                Acc::distribute(Some(body))
            }
            IrTypeDelegate::StringList => Acc {
                io: Some(format!(
                    "final ans = inner.new_StringList_{}(raw.length);
                    for (var i = 0; i < raw.length; i++){{
                        ans.ref.ptr[i] = api2wire_String(raw[i]);
                    }}
                    return ans;",
                    self.context.config.block_index
                )),
                wasm: Some("return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::PrimitiveEnum { ref repr, .. } => {
                format!("return api2wire_{}(raw.index);", repr.safe_ident()).into()
            }
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ref ir) => match ir {
                IrTypeTime::Utc | IrTypeTime::Local | IrTypeTime::Naive => Acc {
                    io: Some("return api2wire_i64(raw.microsecondsSinceEpoch);".into()),
                    wasm: Some("return api2wire_i64(raw.millisecondsSinceEpoch);".into()),
                    ..Default::default()
                },
                IrTypeTime::Duration => Acc {
                    io: Some("return api2wire_i64(raw.inMicroseconds);".into()),
                    wasm: Some("return api2wire_i64(raw.inMilliseconds);".into()),
                    ..Default::default()
                },
            },
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuid => {
                Acc::distribute(Some("return api2wire_uint_8_list(raw.toBytes());".into()))
            }
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuids => Acc::distribute(Some(
                "return api2wire_uint_8_list(api2wireConcatenateBytes(raw));".into(),
            )),
        }
    }

    fn wire2api_body(&self) -> String {
        match &self.ir {
            IrTypeDelegate::Array(array) => match array {
                IrTypeDelegateArray::GeneralArray { general, .. } => format!(
                    r"return {}((raw as List<dynamic>).map(_wire2api_{}).toList());",
                    array.dart_api_type(),
                    general.safe_ident(),
                ),
                IrTypeDelegateArray::PrimitiveArray { .. } => format!(
                    r"return {}(_wire2api_{}(raw));",
                    array.dart_api_type(),
                    array.get_delegate().safe_ident(),
                ),
            },

            IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                IrTypePrimitive::I64 | IrTypePrimitive::U64,
            ) => {
                format!(
                    "return _wire2api_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                )
            }
            IrTypeDelegate::String | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_wire2api_simple_type_cast(&self.ir.dart_api_type())
            }
            IrTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                format!("return {}.values[raw];", ir.dart_api_type())
            }
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeTime::Local | IrTypeTime::Naive | IrTypeTime::Utc => {
                    let is_utc = ir == &IrTypeTime::Naive || ir == &IrTypeTime::Utc;
                    format!("return wire2apiTimestamp(ts: _wire2api_i64(raw), isUtc: {is_utc});")
                }
                IrTypeTime::Duration => "return wire2apiDuration(_wire2api_i64(raw));".to_owned(),
            },
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuid => {
                "return UuidValue.fromByteList(_wire2api_uint_8_list(raw));".to_owned()
            }
            #[cfg(feature = "uuid")]
            IrTypeDelegate::Uuids => "
            final bytes = _wire2api_uint_8_list(raw);
            return wire2apiUuids(bytes);"
                .to_owned(),
        }
    }

    fn structs(&self) -> String {
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
