use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::ir::pack::IrPack;

pub(super) mod cbindgen_executor;
pub(super) mod dummy_function;

pub(super) struct WireCOutputSpec {
    pub code_cbindgen: String,
    pub code_dummy: String,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorWireCInternalConfig,
) -> anyhow::Result<WireCOutputSpec> {
    let code_cbindgen = cbindgen_executor::execute(ir_pack, config)?;
    let code_dummy = dummy_function::generate(config);
    Ok(WireCOutputSpec {
        code_cbindgen,
        code_dummy,
    })
}
