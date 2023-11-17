use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;

impl<'a> WireRustGeneratorWire2apiTrait for DelegateWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        match &self.ir {
            ty @ IrTypeDelegate::StringList => Some(generate_class_from_fields(
                self.ir.clone(),
                &self.context,
                &vec![
                    format!(
                        "ptr: *mut *mut {}",
                        WireRustGenerator::new(ty.get_delegate().clone(), self.context.clone())
                            .rust_wire_type(Target::Io)
                    ),
                    "len: i32".to_owned(),
                ],
            )),
            _ => None,
        }
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        match &self.ir {
            IrTypeDelegate::Array(array) => {
                let acc =
                    Some(
                        format!(
                            "let vec: Vec<{}> = self.wire2api(); support::from_vec_to_array(vec)",
                            array.inner().rust_api_type()
                        ),
                    );
                if array.inner_is_js_value() {
                    return Acc {
                        io: acc,
                        ..Default::default()
                    };
                }
                Acc::distribute(acc)
            },
            IrTypeDelegate::String => {
                Acc {
                    wasm: Some("self".into()),
                    io: Some("let vec: Vec<u8> = self.wire2api(); String::from_utf8_lossy(&vec).into_owned()".into()),
                    ..Default::default()
                }
            },
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                Acc::distribute(Some("ZeroCopyBuffer(self.wire2api())".into()))
            },
            IrTypeDelegate::StringList => Acc {
                io: Some(TypeGeneralListGenerator::WIRE2API_BODY_IO.to_owned()),
                wasm: Some(TypeGeneralListGenerator::WIRE2API_BODY_WASM.to_owned()),
                ..Default::default()
            },
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                let enu = ir.get(self.context.ir_pack);
                let variants =
                    (enu.variants().iter().enumerate())
                        .map(|(idx, variant)| format!("{} => {}::{},", idx, enu.name, variant.name))
                        .collect_vec()
                        .join("\n");
                format!(
                    "match self {{
                        {}
                        _ => unreachable!(\"Invalid variant for {}: {{}}\", self),
                    }}",
                    variants,
                    enu.name
                ).into()
            },
            IrTypeDelegate::Time(ir) => {
                if ir == &IrTypeDelegateTime::Duration {
                    return Acc {
                        io: Some("chrono::Duration::microseconds(self)".into()),
                        wasm: Some("chrono::Duration::milliseconds(self)".into()),
                        ..Default::default()
                    };
                }
                let codegen_timestamp = "let Timestamp { s, ns } = wire2api_timestamp(self);";
                let codegen_naive =
                    "chrono::NaiveDateTime::from_timestamp_opt(s, ns).expect(\"invalid or out-of-range datetime\")";
                let codegen_utc = format!("chrono::DateTime::<chrono::Utc>::from_utc({codegen_naive}, chrono::Utc)");
                let codegen_local = format!("chrono::DateTime::<chrono::Local>::from({codegen_utc})");
                let codegen_conversion = match ir {
                    IrTypeDelegateTime::Naive => codegen_naive,
                    IrTypeDelegateTime::Utc => codegen_utc.as_str(),
                    IrTypeDelegateTime::Local => codegen_local.as_str(),
                    IrTypeDelegateTime::Duration => unreachable!(),
                };
                Acc {
                    common: Some(format!("{codegen_timestamp}{codegen_conversion}")),
                    ..Default::default()
                }
            },
            IrTypeDelegate::TimeList(_) => {
                Acc::distribute(
                    Some(
                        "let vec: Vec<i64> = self.wire2api(); vec.into_iter().map(Wire2Api::wire2api).collect()".into()
                    )
                )
            }
            IrTypeDelegate::Uuid => Acc::distribute(
                Some(
                    "let single: Vec<u8> = self.wire2api(); wire2api_uuid_ref(single.as_slice())".into(),
                ),
            ),
            IrTypeDelegate::Uuids => Acc::distribute(
                Some(
                    "let multiple: Vec<u8> = self.wire2api(); wire2api_uuids(multiple)".into(),
                ),
            ),
            IrTypeDelegate::Backtrace | IrTypeDelegate::Anyhow => "self.wire2api()".into(),
        }
    }
}
