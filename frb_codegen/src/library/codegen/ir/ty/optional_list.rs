crate::ir! {
    #[no_serde]
    pub struct IrTypeOptionalList {
        pub inner: Box<IrType>,
    }
}
