use crate::codegen::generator::wire::c::spec_generator::WireCOutputSpec;

pub(super) fn generate(spec: WireCOutputSpec) -> anyhow::Result<String> {
    Ok(spec.code_cbindgen + &spec.code_dummy)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Concatenates cbindgen output before the generated dummy bundling method.
    #[test]
    fn concatenates_cbindgen_and_dummy_code_in_order() -> anyhow::Result<()> {
        let output = generate(WireCOutputSpec {
            code_cbindgen: "header\n".to_owned(),
            code_dummy: "dummy\n".to_owned(),
        })?;

        assert_eq!(output, "header\ndummy\n");
        Ok(())
    }
}
