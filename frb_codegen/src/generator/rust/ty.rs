use crate::generator::rust::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(TypeRustGeneratorTrait)]
#[derive(Debug, Clone)]
pub enum TypeGenerator {
    Primitive(TypePrimitiveGenerator),
    Delegate(TypeDelegateGenerator),
    PrimitiveList(TypePrimitiveListGenerator),
    Optional(TypeOptionalGenerator),
    GeneralList(TypeGeneralListGenerator),
    StructRef(TypeStructRefGenerator),
    Boxed(TypeBoxedGenerator),
    EnumRef(TypeEnumRefGenerator),
}

#[enum_dispatch]
pub trait TypeRustGeneratorTrait {
    fn wire2api_body(&self) -> String;

    fn wire_struct_fields(&self) -> Vec<String> {
        vec![]
    }

    fn structs(&self) -> String {
        "".to_string()
    }

    fn allocate_funcs(&self, collector: &ExternFuncCollector) -> String {
        "".to_string()
    }

    fn impl_intodart(&self) -> String {
        "".to_string()
    }

    fn new_with_nullptr(&self) -> String {
        "".to_string()
    }

    fn imports(&self) -> Option<String> {
        "".to_string()
    }
}
