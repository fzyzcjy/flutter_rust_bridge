crate::ir! {
pub struct IrDefaultValue {
    pub(crate) mode: IrDefaultValueMode,
    pub(crate) literal: String,
};
}

pub enum IrDefaultValueMode {
    String,
    Others,
}
