mod boxed;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enum_ref;
mod general_list;
mod optional;
mod optional_list;
mod primitive;
mod primitive_list;
mod record;
mod rust_opaque;
mod struct_ref;
mod unencodable;

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

    fn safe_ident(&self) -> String;
}
