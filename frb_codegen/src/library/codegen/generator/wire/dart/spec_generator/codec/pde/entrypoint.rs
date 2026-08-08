use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{
    pde_web_direct_codec, BaseCodecEntrypointTrait, CodecMode, EncodeOrDecode,
};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::entrypoint::CstWireDartCodecEntrypoint;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::entrypoint::generate_serialize_inputs;
use crate::codegen::generator::wire::dart::spec_generator::output_code::{
    DartApiImplClassMethod, WireDartOutputCode,
};
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::generator::wire::rust::spec_generator::misc::function::wire_func_name;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;
use itertools::Itertools;

pub(crate) struct PdeWireDartCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for PdeWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[MirType],
        mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        if mode == EncodeOrDecode::Decode {
            return None;
        }

        let cst =
            super::super::cst::encoder::generate(context.as_wire_dart_codec_cst_context(), types);
        let mut inner = Acc::<Vec<WireDartOutputCode>>::default();
        inner.web.extend(cst.inner.common);
        inner.web.extend(cst.inner.web);

        for func in context
            .mir_pack
            .funcs_with_impl()
            .iter()
            .filter(|func| func.codec_mode_pack.dart2rust == CodecMode::Pde)
            .filter(|func| pde_web_direct_codec(func))
        {
            let signature = generate_platform_call_signature(func);
            inner.io.push(WireDartOutputCode {
                api_impl_class_methods: vec![DartApiImplClassMethod {
                    signature: signature.clone(),
                    body: Some(generate_io_platform_call_body(func)),
                }],
                ..Default::default()
            });
            inner.web.push(WireDartOutputCode {
                api_impl_class_methods: vec![DartApiImplClassMethod {
                    signature,
                    body: Some(generate_web_platform_call_body(func)),
                }],
                ..Default::default()
            });
        }

        Some(WireDartCodecOutputSpec { inner })
    }
}

impl WireDartCodecEntrypointTrait<'_> for PdeWireDartCodecEntrypoint {
    fn generate_dart2rust_inner_func_stmt(&self, func: &MirFunc, _wire_func_name: &str) -> String {
        if !pde_web_direct_codec(func) {
            return generate_io_platform_call_body(func);
        }

        let mut args = func
            .inputs
            .iter()
            .map(|input| input.inner.name.dart_style().to_owned())
            .collect_vec();
        if has_port_argument(func.mode) {
            args.insert(0, "port_".to_owned());
        }
        let maybe_return = if has_port_argument(func.mode) {
            ""
        } else {
            "return "
        };
        format!(
            "{maybe_return}pdeCall{}({});",
            func.id.unwrap(),
            args.join(", ")
        )
    }
}

fn generate_platform_call_signature(func: &MirFunc) -> String {
    let mut params = func
        .inputs
        .iter()
        .map(|input| format!("dynamic {}", input.inner.name.dart_style()))
        .collect_vec();
    if has_port_argument(func.mode) {
        params.insert(0, "NativePortType port_".to_owned());
    }
    format!("dynamic pdeCall{}({})", func.id.unwrap(), params.join(", "))
}

fn generate_io_platform_call_body(func: &MirFunc) -> String {
    let serialize_inputs = generate_serialize_inputs(func);
    let (maybe_port, maybe_return, maybe_bang) = if has_port_argument(func.mode) {
        (", port: port_", "", "")
    } else {
        ("", "return ", "!")
    };
    format!(
        "
        final serializer = SseSerializer(generalizedFrbRustBinding);{serialize_inputs}
        {maybe_return}pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: {}{maybe_port}){maybe_bang};
        ",
        func.id.unwrap()
    )
}

fn generate_web_platform_call_body(func: &MirFunc) -> String {
    CstWireDartCodecEntrypoint.generate_dart2rust_inner_func_stmt(func, &wire_func_name(func))
}
