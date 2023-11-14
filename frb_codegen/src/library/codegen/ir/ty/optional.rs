crate::ir! {
#[no_serde]
pub struct IrTypeOptional {
    pub inner: Box<IrType>,
}
}
