use crate::codegen::ir::ty::IrType;

crate::ir! {
/// A `Result<T, E>` or a direct type `T`
pub(crate) struct IrMaybeResult {
    pub(crate) normal: IrType,
    pub(crate) error: Option<IrType>,
}
}
