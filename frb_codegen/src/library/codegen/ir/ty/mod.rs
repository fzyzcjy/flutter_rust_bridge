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

use crate::codegen::ir::pack::IrPack;
use enum_dispatch::enum_dispatch;

crate::ir! {
// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(IrTypeTrait)]
pub enum IrType {
    // alphabetical order
    Boxed(boxed::IrTypeBoxed),
    DartOpaque(dart_opaque::IrTypeDartOpaque),
    Delegate(delegate::IrTypeDelegate),
    Dynamic(dynamic::IrTypeDynamic),
    EnumRef(enumeration::IrTypeEnumRef),
    GeneralList(general_list::IrTypeGeneralList),
    Optional(optional::IrTypeOptional),
    OptionalList(optional_list::IrTypeOptionalList),
    Primitive(primitive::IrTypePrimitive),
    PrimitiveList(primitive_list::IrTypePrimitiveList),
    Record(record::IrTypeRecord),
    RustOpaque(rust_opaque::IrTypeRustOpaque),
    StructRef(structure::IrTypeStructRef),
    Unencodable(unencodable::IrTypeUnencodable),
}
}

impl IrType {
    pub fn visit_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        if f(self) {
            return;
        }
        self.visit_children_types(f, ir_pack);
    }

    #[inline]
    pub fn is_struct_or_enum_or_record(&self) -> bool {
        matches!(
            self,
            IrType::StructRef(_) | IrType::EnumRef(_) | IrType::Record(_)
        )
    }
}

#[enum_dispatch]
pub trait IrTypeTrait {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack);

    /// A string that can be used as an identifier safely, i.e. without any special characters inside
    fn safe_ident(&self) -> String;
}
