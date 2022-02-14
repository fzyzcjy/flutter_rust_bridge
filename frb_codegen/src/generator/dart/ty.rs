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
    // TODO
}
