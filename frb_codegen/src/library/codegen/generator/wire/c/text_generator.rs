use crate::codegen::generator::wire::c::spec_generator::WireCOutputSpec;

pub(super) fn generate(spec: WireCOutputSpec) -> anyhow::Result<String> {
    Ok(spec.code_cbindgen + &spec.code_dummy)
}
