use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArrayMode, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartTransferCstGeneratorEncoderTrait for DelegateWireDartTransferCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        match &self.ir {
            IrTypeDelegate::Array(ref array) => match &array.mode {
                IrTypeDelegateArrayMode::General(_) => Acc::distribute(Some(format!(
                    "return cst_encode_{}(raw);",
                    array.get_delegate().safe_ident(),
                ))),
                IrTypeDelegateArrayMode::Primitive(_) => Acc {
                    io: Some(format!(
                        "final ans = wire.new_{}({length});
                        ans.ref.ptr.asTypedList({length}).setAll(0, raw);
                        return ans;",
                        array.get_delegate().safe_ident(),
                        length = array.length,
                    )),
                    wasm: Some(format!(
                        "return {}.fromList(raw);",
                        ApiDartGenerator::new(
                            array.get_delegate(),
                            self.context.as_api_dart_context()
                        )
                        .dart_api_type()
                    )),
                    ..Default::default()
                },
            },

            IrTypeDelegate::String => Acc {
                io: Some(format!(
                    "return cst_encode_{}(utf8.encoder.convert(raw));",
                    uint8list_safe_ident()
                )),
                wasm: Some("return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                // In this case, even though the body is the same, their types are different
                // and must be split.
                let body = format!(
                    "return cst_encode_{}(raw);",
                    self.ir.get_delegate().safe_ident()
                );
                Acc::distribute(Some(body))
            }
            IrTypeDelegate::StringList => Acc {
                io: Some(
                    "final ans = wire.new_StringList(raw.length);
                    for (var i = 0; i < raw.length; i++){
                        ans.ref.ptr[i] = cst_encode_String(raw[i]);
                    }
                    return ans;"
                        .to_string(),
                ),
                wasm: Some("return raw;".into()),
                ..Default::default()
            },
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ref repr, .. }) => {
                format!("return cst_encode_{}(raw.index);", repr.safe_ident()).into()
            }
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeDelegateTime::Utc | IrTypeDelegateTime::Local | IrTypeDelegateTime::Naive => {
                    Acc {
                        io: Some("return cst_encode_i_64(raw.microsecondsSinceEpoch);".into()),
                        wasm: Some("return cst_encode_i_64(raw.millisecondsSinceEpoch);".into()),
                        ..Default::default()
                    }
                }
                IrTypeDelegateTime::Duration => Acc {
                    io: Some("return cst_encode_i_64(raw.inMicroseconds);".into()),
                    wasm: Some("return cst_encode_i_64(raw.inMilliseconds);".into()),
                    ..Default::default()
                },
            },
            IrTypeDelegate::TimeList(t) => Acc::distribute(Some(format!(
                "final ans = Int64List(raw.length);
                for (var i=0; i < raw.length; ++i) ans[i] = cst_encode_{}(raw[i]);
                return cst_encode_list_prim_i_64(ans);",
                IrTypeDelegate::Time(*t).safe_ident()
            ))),
            IrTypeDelegate::Uuid => Acc::distribute(Some(format!(
                "return cst_encode_{}(raw.toBytes());",
                uint8list_safe_ident()
            ))),
            IrTypeDelegate::Uuids => Acc::distribute(Some(format!(
                "final builder = BytesBuilder();
                for (final element in raw) {{
                  builder.add(element.toBytes());
                }}
                return cst_encode_{}(builder.toBytes());",
                uint8list_safe_ident()
            ))),
            IrTypeDelegate::Backtrace => unimplemented!(),
            IrTypeDelegate::Anyhow => unimplemented!(),
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match (&self.ir, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Wasm) => "List<String>".into(),
            (IrTypeDelegate::StringList, _) => "ffi.Pointer<wire_StringList>".to_owned(),
            _ => WireDartTransferCstGenerator::new(self.ir.get_delegate(), self.context)
                .dart_wire_type(target),
        }
    }
}

fn uint8list_safe_ident() -> String {
    IrTypePrimitiveList {
        primitive: IrTypePrimitive::U8,
    }
    .safe_ident()
}
