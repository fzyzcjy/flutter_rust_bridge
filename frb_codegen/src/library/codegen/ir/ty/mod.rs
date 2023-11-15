pub(crate) mod boxed;
pub(crate) mod dart_opaque;
pub(crate) mod delegate;
pub(crate) mod dynamic;
pub(crate) mod enumeration;
pub(crate) mod general_list;
pub(crate) mod optional;
pub(crate) mod optional_list;
pub(crate) mod primitive;
pub(crate) mod primitive_list;
pub(crate) mod record;
pub(crate) mod rust_opaque;
pub(crate) mod structure;
pub(crate) mod unencodable;

use enum_dispatch::enum_dispatch;
use primitive::IrTypePrimitive;

crate::ir! {
// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(IrTypeTrait)]
pub enum IrType {
    // alphabetical order
    Boxed(IrTypeBoxed),
    DartOpaque(IrTypeDartOpaque),
    Delegate(IrTypeDelegate),
    Dynamic(IrTypeDynamic),
    EnumRef(IrTypeEnumRef),
    GeneralList(IrTypeGeneralList),
    Optional(IrTypeOptional),
    OptionalList(IrTypeOptionalList),
    Primitive(IrTypePrimitive),
    PrimitiveList(IrTypePrimitiveList),
    Record(IrTypeRecord),
    RustOpaque(IrTypeRustOpaque),
    StructRef(IrTypeStructRef),
    Unencodable(IrTypeUnencodable),
}
}

impl IrType {
    pub fn visit_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        if f(self) {
            return;
        }
        self.visit_children_types(f, ir_pack);
    }
}

#[enum_dispatch]
pub trait IrTypeTrait {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack);

    fn safe_ident(&self) -> String;
}
