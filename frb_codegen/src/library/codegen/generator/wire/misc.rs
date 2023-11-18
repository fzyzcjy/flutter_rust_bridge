use crate::codegen::ir::func::IrFuncMode;

// This is related to the specific `wire` implementation, thus put in `wire` module,
// instead of the `ir` module.
pub(crate) fn has_port_argument(mode: IrFuncMode) -> bool {
    !matches!(mode, IrFuncMode::Sync)
}
