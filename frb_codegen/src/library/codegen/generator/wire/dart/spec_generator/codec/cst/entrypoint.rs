use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

pub(crate) struct CstWireDartCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireDartGeneratorContext<'_>, WireDartCodecOutputSpec>
    for CstWireDartCodecEntrypoint
{
    fn generate(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
        mode: EncodeOrDecode,
    ) -> Option<WireDartCodecOutputSpec> {
        match mode {
            EncodeOrDecode::Encode => Some(super::encoder::generate(
                context.as_wire_dart_codec_cst_context(),
                types,
            )),
            EncodeOrDecode::Decode => None,
        }
    }
}

impl WireDartCodecEntrypointTrait<'_> for CstWireDartCodecEntrypoint {
    fn generate_dart2rust_inner_func_stmt(&self, func: &IrFunc, wire_func_name: &str) -> String {
        let prepare_stmts = func
            .inputs
            .iter()
            .enumerate()
            .map(|(index, input)| {
                format!(
                    "var arg{index} = cst_encode_{ty_ident}({name});",
                    ty_ident = input.ty.safe_ident(),
                    name = &input.name.dart_style()
                )
            })
            .collect_vec();
        let params = [
            if has_port_argument(func.mode) {
                vec!["port_".to_owned()]
            } else {
                vec![]
            },
            (0..prepare_stmts.len())
                .map(|index| format!("arg{index}"))
                .collect_vec(),
        ]
        .concat();
        format!(
            "{}
            return {wire_func_name}({});
            ",
            prepare_stmts.join("\n"),
            params.join(", ")
        )
    }
}
