use crate::codegen::config::internal_config::ControllerInternalConfig;

pub(super) fn run(
    config: &ControllerInternalConfig,
    run_inner: &impl Fn() -> anyhow::Result<()>,
) -> anyhow::Result<()> {
    if config.watch {
        run_watch(run_inner)
    } else {
        run_inner()
    }
}

fn run_watch(run_inner: &impl Fn() -> anyhow::Result<()>) -> anyhow::Result<()> {
    todo!()
}
