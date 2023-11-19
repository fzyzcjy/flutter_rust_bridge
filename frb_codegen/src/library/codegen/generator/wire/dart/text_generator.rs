use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::WireDartOutputSpec;

pub(super) struct WireDartOutputText {
    pub(super) text: Acc<Option<String>>,
}

pub(super) fn generate(spec: &WireDartOutputSpec) -> anyhow::Result<WireDartOutputText> {
    // TODO below is only a super simple incomplete version, should update it later
    Ok(WireDartOutputText { text })
}
