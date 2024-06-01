use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, EncodeOrDecode};
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::base::{
    WireRustCodecEntrypointTrait, WireRustCodecOutputSpec,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::WireRustCodecCstGenerator;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::entrypoint::create_maybe_port_param;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;
use std::convert::TryInto;

pub(crate) struct CstWireRustCodecEntrypoint;

impl BaseCodecEntrypointTrait<WireRustGeneratorContext<'_>, WireRustCodecOutputSpec>
    for CstWireRustCodecEntrypoint
{
    fn generate(
        &self,
        context: WireRustGeneratorContext,
        types: &[MirType],
        mode: EncodeOrDecode,
    ) -> Option<WireRustCodecOutputSpec> {
        match mode {
            EncodeOrDecode::Encode => None,
            EncodeOrDecode::Decode => Some(super::decoder::generate(
                context.as_wire_rust_codec_cst_context(),
                types,
            )),
        }
    }
}

impl WireRustCodecEntrypointTrait<'_> for CstWireRustCodecEntrypoint {
    fn generate_func_params(
        &self,
        func: &MirFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>> {
        let mut params = Acc::new(|target| {
            (create_maybe_port_param(func.mode, target).into_iter()).collect_vec()
        });

        params += func
            .inputs
            .iter()
            .map(|field| {
                let name = field.inner.name.rust_style().to_owned();
                Acc::new(|target| match target {
                    TargetOrCommon::Common => ExternFuncParam {
                        name: name.clone(),
                        rust_type: format!(
                            "impl CstDecode<{}>",
                            WireRustCodecCstGenerator::new(
                                field.inner.ty.clone(),
                                context.as_wire_rust_codec_cst_context()
                            )
                            .generate_wire_func_param_api_type()
                            .unwrap_or(field.inner.ty.rust_api_type())
                        ),
                        dart_type: "THIS_TYPE_SHOULD_NOT_BE_USED".into(),
                    },
                    TargetOrCommon::Io | TargetOrCommon::Web => {
                        let target: Target = target.try_into().unwrap();
                        ExternFuncParam::new(
                            name.clone(),
                            target,
                            &field.inner.ty,
                            context.as_wire_rust_codec_cst_context(),
                        )
                    }
                })
            })
            .collect();

        params
    }

    fn generate_func_call_decode(
        &self,
        func: &MirFunc,
        context: WireRustGeneratorContext,
    ) -> String {
        func.inputs
            .iter()
            .map(|field| {
                let gen = WireRustGenerator::new(field.inner.ty.clone(), context);

                let name = field.inner.name.rust_style();
                let mut expr = format!("{name}.cst_decode()");
                if let Some(wrapper) = gen.generate_wire_func_call_decode_wrapper() {
                    expr = format!("{wrapper}({expr})");
                }

                format!("let api_{name} = {expr};")
            })
            .join("")
    }
}
