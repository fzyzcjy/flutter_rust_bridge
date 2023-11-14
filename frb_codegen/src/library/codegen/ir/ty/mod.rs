use enum_dispatch::enum_dispatch;

crate::ir! {
// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(IrTypeTrait)]
pub enum IrType {
    Primitive(IrTypePrimitive),
    Delegate(IrTypeDelegate),
    PrimitiveList(IrTypePrimitiveList),
    Optional(IrTypeOptional),
    OptionalList(IrTypeOptionalList),
    GeneralList(IrTypeGeneralList),
    StructRef(IrTypeStructRef),
    Boxed(IrTypeBoxed),
    EnumRef(IrTypeEnumRef),
    SyncReturn(IrTypeSyncReturn),
    DartOpaque(IrTypeDartOpaque),
    RustOpaque(IrTypeRustOpaque),
    Dynamic(IrTypeDynamic),
    Record(IrTypeRecord),
    Unencodable(IrTypeUnencodable),
}
}

impl IrType {
    pub fn visit_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        if f(self) {
            return;
        }
        self.visit_children_types(f, ir_file);
    }
}

#[enum_dispatch]
pub trait IrTypeTrait {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile);
}
