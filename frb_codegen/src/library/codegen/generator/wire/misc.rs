use crate::codegen::ir::mir::func::MirFuncMode;

// This is related to the specific `wire` implementation, thus put in `wire` module,
// instead of the `ir` module.
pub(crate) fn has_port_argument(mode: MirFuncMode) -> bool {
    !matches!(mode, MirFuncMode::Sync)
}
