use crate::codegen::generator::misc::codec::BaseCodecEntrypointTrait;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypointTrait, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::WireDartCodecDcoGeneratorContext;
use crate::codegen::generator::wire::misc::has_port_argument;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

pub(crate) struct CstWireDartCodecEntrypoint {}

impl BaseCodecEntrypointTrait<WireDartGeneratorContext, Box<dyn WireDartCodecOutputSpec>>
    for CstWireDartCodecEntrypoint
{
    fn generate_encode(
        &self,
        context: WireDartGeneratorContext,
        types: &[IrType],
    ) -> Option<Box<dyn WireDartCodecOutputSpec>> {
        Some(Box::new(encoder::generate(
            context.as_wire_dart_codec_cst_context(),
            types,
        )))
    }

    fn generate_decode(
        &self,
        _context: WireDartGeneratorContext,
        _types: &[IrType],
    ) -> Option<Box<dyn WireDartCodecOutputSpec>> {
        None
    }
}

impl WireDartCodecEntrypointTrait for CstWireDartCodecEntrypoint {
    fn generate_dart2rust_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
        func.inputs
            .iter()
            .enumerate()
            .map(|(index, input)| {
                format!(
                    "var arg{index} = cst_encode_{ty_ident}({name});",
                    ty_ident = input.ty.safe_ident(),
                    name = &input.name.dart_style()
                )
            })
            .collect_vec()
    }

    fn generate_dart2rust_func_wire_param_list(
        &self,
        func: &IrFunc,
        num_prepare_args: usize,
    ) -> Vec<String> {
        [
            if has_port_argument(func.mode) {
                vec!["port_".to_owned()]
            } else {
                vec![]
            },
            (0..num_prepare_args)
                .map(|index| format!("arg{index}"))
                .collect_vec(),
        ]
        .concat()
    }

    fn generate_rust2dart_codec_object(&self, func: &IrFunc) -> String {
        unreachable!()
    }
}
