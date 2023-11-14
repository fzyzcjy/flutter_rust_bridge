crate::ir! {
pub struct IrTypeRecord {
    /// Refers to a virtual struct definition.
    pub inner: IrTypeStructRef,
    pub values: Box<[IrType]>,
}
}
