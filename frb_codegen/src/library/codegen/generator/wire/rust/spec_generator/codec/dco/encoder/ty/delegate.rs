use crate::codegen::generator::codec::sse::ty::delegate::{
    encode_std_duration, encode_std_system_time,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::enumeration::{
    generate_enum_access_object_core, parse_wrapper_name_into_dart_name_and_self_path,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::delegate::{
    MirTypeDelegate, MirTypeDelegatePrimitiveEnum, MirTypeDelegateTime,
};
use itertools::Itertools;

impl WireRustCodecDcoGeneratorEncoderTrait for DelegateWireRustCodecDcoGenerator<'_> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn generate_impl_into_dart(&self) -> Option<String> {
        // frb-coverage:ignore-end
        match &self.mir {
            MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { mir, .. }) => {
                let src = mir.get(self.context.mir_pack);
                let (name, self_path) =
                    parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

                let self_ref =
                    generate_enum_access_object_core(mir, "self".to_owned(), self.context);
                let variants = src
                    .variants()
                    .iter()
                    .enumerate()
                    .map(|(idx, variant)| {
                        format!("{}::{} => {}.into_dart(),", self_path, variant.name, idx)
                    })
                    .collect_vec()
                    .join("\n");

                let body = format!(
                    "match {self_ref} {{
                    {variants}
                    _ => unreachable!(),
                }}"
                );

                Some(
                    generate_impl_into_dart(&name, &body)
                        + &generate_impl_into_into_dart(&src.name.rust_style(), &src.wrapper_name),
                )
            }
            MirTypeDelegate::CustomSerDes(inner) => {
                let name = inner.info.cleared_rust_api_type();
                let wrapper_name = format!("FrbWrapper<{name}>");
                let body = format!(
                    "{}(self.0).into_dart()",
                    inner.info.rust2dart.rust_function.rust_style()
                );
                Some(
                    generate_impl_into_dart(&wrapper_name, &body)
                        + &generate_impl_into_into_dart(&name, &Some(wrapper_name.clone())),
                )
            }
            MirTypeDelegate::Time(mir) => match mir {
                MirTypeDelegateTime::StdSystemTime => Some(generate_impl_into_dart_for_time(
                    "std::time::SystemTime",
                    &encode_std_system_time_for_dco("self.0"),
                )),
                MirTypeDelegateTime::StdInstant => Some(generate_impl_into_dart_for_time(
                    "std::time::Instant",
                    &encode_std_instant_for_dco("self.0"),
                )),
                MirTypeDelegateTime::StdDuration => Some(generate_impl_into_dart_for_time(
                    "std::time::Duration",
                    &encode_std_duration_for_dco("self.0"),
                )),
                MirTypeDelegateTime::TokioInstant => Some(generate_impl_into_dart_for_time(
                    "tokio::time::Instant",
                    &encode_tokio_instant_for_dco("self.0"),
                )),
                _ => None,
            },
            _ => None,
        }
    }
}

fn generate_impl_into_dart_for_time(name: &str, body: &str) -> String {
    let wrapper_name = format!("FrbWrapper<{name}>");
    let body = format!(
        "let value: i64 = {body};
        value.into_dart()"
    );
    generate_impl_into_dart(&wrapper_name, &body)
        + &generate_impl_into_into_dart(name, &Some(wrapper_name))
}

fn encode_std_duration_for_dco(value: &str) -> String {
    format!(
        r#"{{
            #[cfg(target_arch = "wasm32")]
            {{
                {value}.as_millis().try_into().expect("cannot get milliseconds from time")
            }}
            #[cfg(not(target_arch = "wasm32"))]
            {{
                {micros}
            }}
        }}"#,
        micros = encode_std_duration(value)
    )
}

fn encode_std_system_time_for_dco(value: &str) -> String {
    format!(
        r#"{{
            #[cfg(target_arch = "wasm32")]
            {{
                match {value}.duration_since(std::time::SystemTime::UNIX_EPOCH) {{
                    Ok(duration) => duration.as_millis().try_into().expect("cannot get milliseconds from time"),
                    Err(error) => {{
                        let millis = i128::try_from(error.duration().as_millis()).expect("cannot get milliseconds from time");
                        i64::try_from(-millis).expect("cannot get milliseconds from time")
                    }},
                }}
            }}
            #[cfg(not(target_arch = "wasm32"))]
            {{
                {micros}
            }}
        }}"#,
        micros = encode_std_system_time(value)
    )
}

fn encode_std_instant_for_dco(value: &str) -> String {
    format!(
        r#"{{
            let value = {value}.clone();
            let now_instant = std::time::Instant::now();
            let now_system_time = std::time::SystemTime::now();
            let system_time = if value >= now_instant {{
                now_system_time.checked_add(value.duration_since(now_instant)).expect("instant out of range")
            }} else {{
                now_system_time.checked_sub(now_instant.duration_since(value)).expect("instant out of range")
            }};
            {system_time}
        }}"#,
        system_time = encode_std_system_time_for_dco("system_time")
    )
}

fn encode_tokio_instant_for_dco(value: &str) -> String {
    encode_std_instant_for_dco(&format!("{value}.into_std()"))
}
