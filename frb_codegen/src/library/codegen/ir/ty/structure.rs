// Call it "structure" not "struct", since the latter is a preserved word.
crate::ir! {
pub struct IrTypeStructRef {
    pub name: String,
    pub freezed: bool,
    pub empty: bool,
    pub is_exception: bool,
}
}
