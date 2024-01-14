use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::entrypoint::SseWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::generator::wire::rust::spec_generator::misc::wire_func::wire_func_name;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

pub(crate) struct PdeWireRustCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for PdeWireRustCodecEntrypoint
{
    fn generate(
        &self,
        context: WireRustGeneratorContext,
        types: &[IrType],
        mode: EncodeOrDecode,
    ) -> Option<WireRustCodecOutputSpec> {
        match mode {
            EncodeOrDecode::Encode => None,
            EncodeOrDecode::Decode => Some(generate_func_call_dispatcher(&context.ir_pack.funcs)),
        }
    }
}

fn generate_func_call_dispatcher(funcs: &[IrFunc]) -> WireRustCodecOutputSpec {
    let variants = (funcs.iter())
        .map(|f| format!("{} => {},\n", TODO, wire_func_name(f)))
        .join("");
    let code = format!(
        "
        fn pde_ffi_dispatcher(
            func_id_: i32,
            port_: flutter_rust_bridge::for_generated::MessagePort,
            ptr_: flutter_rust_bridge::for_generated::PlatformGeneralizedUint8ListPtr,
            rust_vec_len_: i32,
            data_len_: i32,
        ) {{
            match func_id_ {{
                {variants}
            }}
        }}
        "
    );
    WireRustCodecOutputSpec {
        inner: Acc::new_common(vec![code.into()]),
    }
}

impl WireRustCodecEntrypointTrait<'_> for PdeWireRustCodecEntrypoint {
    fn generate_func_params(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        SseWireRustCodecEntrypoint.generate_func_params(func, context)
    }

    fn generate_func_call_decode(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> String {
        SseWireRustCodecEntrypoint.generate_func_call_decode(func, context)
    }
}
