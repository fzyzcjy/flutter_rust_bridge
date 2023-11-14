crate::ir! {
pub struct IrTypeBoxed {
    /// if false, means that we automatically add it when transforming it - it does not exist in real api.
    pub exist_in_real_api: bool,
    pub inner: Box<IrType>,
}
}
