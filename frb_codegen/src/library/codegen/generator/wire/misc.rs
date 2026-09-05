use crate::codegen::ir::mir::func::MirFuncMode;

// This is related to the specific `wire` implementation, thus put in `wire` module,
// instead of the `ir` module.
pub(crate) fn has_port_argument(mode: MirFuncMode) -> bool {
    !matches!(mode, MirFuncMode::Sync)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps ports for asynchronous functions but omits them for synchronous ones.
    #[test]
    fn distinguishes_function_modes_that_require_ports() {
        assert!(has_port_argument(MirFuncMode::Normal));
        assert!(!has_port_argument(MirFuncMode::Sync));
    }
}
