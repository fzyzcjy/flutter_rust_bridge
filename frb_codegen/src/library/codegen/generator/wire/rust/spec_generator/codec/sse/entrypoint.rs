use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::body::generate_encode_or_decode;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

pub(crate) struct SseWireRustCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for SseWireRustCodecEntrypoint
{
    fn generate(
        &self,
        context: WireRustGeneratorContext,
        types: &[IrType],
        mode: EncodeOrDecode,
    ) -> Option<WireRustCodecOutputSpec> {
        Some(generate_encode_or_decode(
            context.as_wire_rust_codec_sse_context(),
            types,
            mode,
        ))
    }
}

impl WireRustCodecEntrypointTrait<'_> for SseWireRustCodecEntrypoint {
    fn generate_func_params(
        &self,
        func: &IrFunc,
        _context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        let mut params = vec![
            ExternFuncParam {
                name: "ptr_".to_owned(),
                rust_type: "*mut u8".to_owned(),
                dart_type: "ffi.Pointer<ffi.Uint8>".to_owned(),
            },
            ExternFuncParam {
                name: "rust_vec_len_".to_owned(),
                rust_type: "i32".to_owned(),
                dart_type: "int".to_owned(),
            },
            ExternFuncParam {
                name: "data_len_".to_owned(),
                rust_type: "i32".to_owned(),
                dart_type: "int".to_owned(),
            },
        ];

        if has_port_argument(func.mode) {
            params.insert(
                0,
                ExternFuncParam {
                    name: "port_".to_owned(),
                    rust_type: "i64".to_owned(),
                    dart_type: "int".to_owned(),
                },
            );
        }

        Acc::new(|_| params)
    }

    fn generate_func_call_decode(
        &self,
        func: &IrFunc,
        context: WireRustGeneratorContext,
    ) -> String {
        let primary = (func.inputs.iter())
            .map(|field| {
                let name = field.name.rust_style();
                let rust_api_type = field.ty.rust_api_type();
                format!("let api_{name} = {rust_api_type}::sse_decode(&mut deserializer);")
            })
            .join("\n");
        format!(
            "
            let mut deserializer = unsafe {{ flutter_rust_bridge::for_generated::SseDeserializer::from_wire(ptr_, rust_vec_len_, data_len_) }};
            {primary}
            deserializer.end();"
        )
    }
}
