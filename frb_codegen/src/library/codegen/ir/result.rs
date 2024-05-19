use crate::codegen::ir::ty::{IrContext, IrType};
use crate::library::codegen::ir::ty::IrTypeTrait;

crate::ir! {
/// A `Result<T, E>` or a direct type `T`
pub(crate) struct IrMaybeResult {
    pub(crate) normal: IrType,
    pub(crate) error: Option<IrType>,
    /// Combines `normal` and `error` into a new synthesized type
    pub(crate) delegate: IrType,
}
}

impl IrMaybeResult {
    pub(crate) fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.normal.visit_types(f, ir_context);
        if let Some(error) = &self.error {
            error.visit_types(f, ir_context);
        }
        self.delegate.visit_types(f, ir_context);
    }

    pub(crate) fn safe_ident(&self) -> String {
        self.delegate.safe_ident()
    }

    pub(crate) fn rust_api_type(&self) -> String {
        if let Some(error) = &self.error {
            format!(
                "std::result::Result<{}, {}>",
                self.normal.rust_api_type(),
                error.rust_api_type()
            )
        } else {
            self.normal.rust_api_type()
        }
    }
}
