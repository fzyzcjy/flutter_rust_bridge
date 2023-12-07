use crate::codegen::generator::wire::dart::spec_generator::transfer::base::WireDartTransferEntrypointTrait;
use crate::codegen::ir::func::IrFunc;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

pub(crate) struct CstWireDartTransferEntrypoint {}

impl WireDartTransferEntrypointTrait for CstWireDartTransferEntrypoint {
    fn generate_func_stmt_prepare_args(&self, func: &IrFunc) -> Vec<String> {
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
}
