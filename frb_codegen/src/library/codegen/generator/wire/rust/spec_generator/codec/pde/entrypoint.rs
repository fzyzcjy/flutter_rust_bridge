use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{
    BaseCodecEntrypointTrait, CodecMode, EncodeOrDecode,
};
use crate::codegen::generator::misc::comments::generate_codec_comments;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::entrypoint::SseWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::generator::wire::rust::spec_generator::misc::function::wire_func_name;
use crate::codegen::ir::mir::func::{MirFunc, MirFuncMode};
use crate::codegen::ir::mir::ty::MirType;
use itertools::Itertools;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

pub(crate) struct PdeWireRustCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for PdeWireRustCodecEntrypoint
{
    fn generate(
        &self,
        context: WireRustGeneratorContext,
        _types: &[MirType],
        mode: EncodeOrDecode,
    ) -> Option<WireRustCodecOutputSpec> {
        match mode {
            EncodeOrDecode::Encode => None,
            EncodeOrDecode::Decode => {
                Some(generate_ffi_dispatcher(&context.mir_pack.funcs_with_impl()))
            }
        }
    }
}

fn generate_ffi_dispatcher(funcs: &[MirFunc]) -> WireRustCodecOutputSpec {
    let variants = FfiDispatcherMode::iter()
        .map(|mode| {
            (
                mode,
                (funcs.iter())
                    .filter(|f| f.codec_mode_pack.dart2rust == CodecMode::Pde)
                    .filter(|f| FfiDispatcherMode::from(&f.mode) == mode)
                    .map(|f| {
                        let maybe_port = if has_port_argument(f.mode) {
                            "port, "
                        } else {
                            ""
                        };
                        format!(
                            "{} => {}_impl({maybe_port}ptr, rust_vec_len, data_len),",
                            f.id.unwrap(),
                            wire_func_name(f)
                        )
                    })
                    .join("\n"),
            )
        })
        .collect();
    let code = generate_ffi_dispatcher_raw(&variants, "flutter_rust_bridge");
    WireRustCodecOutputSpec {
        inner: Acc::new_common(vec![code.into()]),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumIter, Display, Hash)]
pub(crate) enum FfiDispatcherMode {
    Primary,
    Sync,
}

impl From<&MirFuncMode> for FfiDispatcherMode {
    fn from(value: &MirFuncMode) -> Self {
        match value {
            MirFuncMode::Normal => Self::Primary,
            MirFuncMode::Sync => Self::Sync,
        }
    }
}

pub(crate) fn generate_ffi_dispatcher_raw(
    variants: &HashMap<FfiDispatcherMode, String>,
    crate_name: &str,
) -> String {
    FfiDispatcherMode::iter()
        .map(|mode| {
            let (name, maybe_port, maybe_return) = match mode {
                FfiDispatcherMode::Primary => (
                    "primary",
                    format!("port: {crate_name}::for_generated::MessagePort,"),
                    "".to_owned(),
                ),
                FfiDispatcherMode::Sync => (
                    "sync",
                    "".to_owned(),
                    format!("-> {crate_name}::for_generated::WireSyncRust2DartSse"),
                ),
            };
            let variants = &variants[&mode];
            let comments = generate_codec_comments(CodecMode::Pde);

            format!(
                "
                fn pde_ffi_dispatcher_{name}_impl(
                    func_id: i32,{maybe_port}
                    ptr: {crate_name}::for_generated::PlatformGeneralizedUint8ListPtr,
                    rust_vec_len: i32,
                    data_len: i32,
                ) {maybe_return} {{
                    {comments}
                    match func_id {{
                        {variants}
                        _ => unreachable!(),
                    }}
                }}
                ",
            )
        })
        .join("")
}

impl WireRustCodecEntrypointTrait<'_> for PdeWireRustCodecEntrypoint {
    fn generate_func_params(
        &self,
        func: &MirFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        SseWireRustCodecEntrypoint.generate_func_params(func, context)
    }

    fn generate_func_call_decode(
        &self,
        func: &MirFunc,
        context: WireRustGeneratorContext,
    ) -> String {
        SseWireRustCodecEntrypoint.generate_func_call_decode(func, context)
    }
}
