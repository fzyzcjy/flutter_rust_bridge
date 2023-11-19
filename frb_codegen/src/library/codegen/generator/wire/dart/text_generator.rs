use crate::codegen::generator::acc::Acc;

pub(super) struct WireDartOutputText {
    pub(super) text: Acc<Option<String>>,
}

pub(super) fn generate() -> anyhow::Result<WireDartOutputText> {
    // TODO below is only a super simple incomplete version, should update it later
    Ok(WireDartOutputText { text })
}
