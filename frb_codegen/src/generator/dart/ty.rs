use crate::generator::dart::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(TypeDartGeneratorTrait)]
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
pub trait TypeDartGeneratorTrait {
    fn api2wire_body(&self) -> String;

    fn api_fill_to_wire_body(&self) -> String {
        "".to_string()
    }

    fn wire2api_body(&self) -> String;

    fn structs(&self) -> String {
        "".to_string()
    }
}

impl TypeGenerator {
    pub fn new(ty: IrType) -> TypeGenerator {
        match ty {
            Primitive(inner) => TypePrimitiveGenerator(inner).into(),
            Delegate(inner) => TypeDelegateGenerator(inner).into(),
            PrimitiveList(inner) => TypePrimitiveListGenerator(inner).into(),
            Optional(inner) => TypeOptionalGenerator(inner).into(),
            GeneralList(inner) => TypeGeneralListGenerator(inner).into(),
            StructRef(inner) => TypeStructRefGenerator(inner).into(),
            Boxed(inner) => TypeBoxedGenerator(inner).into(),
            EnumRef(inner) => TypeEnumRefGenerator(inner).into(),
        }
    }
}
