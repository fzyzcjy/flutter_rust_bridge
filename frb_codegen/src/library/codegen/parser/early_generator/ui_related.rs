use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::parser::hir::flat::extra_code_injector::{
    inject_extra_codes, InjectExtraCodeBlock,
};
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;

pub(crate) fn generate(
    pack: &mut IrEarlyGeneratorPack,
    config_mir: &ParserMirInternalConfig,
) -> anyhow::Result<()> {
    if !should_enable_ui(pack)? {
        return Ok(());
    }

    let output_namespace = &(config_mir.rust_input_namespace_pack).rust_output_path_namespace;
    inject_extra_codes(
        &mut pack.hir_flat_pack,
        output_namespace,
        &[InjectExtraCodeBlock {
            code: generate_boilerplate(),
            should_parse: true,
        }],
    )?;

    Ok(())
}

fn should_enable_ui(pack: &mut IrEarlyGeneratorPack) -> anyhow::Result<bool> {
    for ty in &pack.hir_flat_pack.structs {
        let attr = FrbAttributes::parse(&ty.src.attrs)?;
        if attr.ui_state() {
            return Ok(true);
        }
    }
    Ok(false)
}

fn generate_boilerplate() -> String {
    r#"
#[flutter_rust_bridge::frb(opaque)]
#[derive(Default)]
pub struct BaseState {
    notify_ui: Option<StreamSink<()>>,
}

impl BaseState {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> Self {
        Self { notify_ui: None }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn create_notify_ui_stream(&mut self, sink: StreamSink<()>) {
        self.notify_ui = Some(sink);
    }

    #[flutter_rust_bridge::frb(ignore)]
    pub(crate) fn notify_ui(&self) {
        self.notify_ui.as_ref().unwrap().add(()).unwrap()
    }
}
    "#
    .to_owned()
}
