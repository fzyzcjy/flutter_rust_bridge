// Call it "enumeration" not "enum", since the latter is a preserved word.

crate::ir! {
pub struct IrTypeEnumRef {
    pub name: String,
    pub is_exception: bool,
}
}
