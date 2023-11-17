use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::enumeration::IrTypeEnumRef;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::optional::IrTypeOptional;
use crate::codegen::ir::ty::optional_list::IrTypeOptionalList;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::record::IrTypeRecord;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::*;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;
use paste::paste;

#[enum_dispatch(WireRustGeneratorInfoTrait)]
#[enum_dispatch(WireRustGeneratorApi2wireTrait)]
#[enum_dispatch(WireRustGeneratorWire2apiTrait)]
#[enum_dispatch(WireRustGeneratorCommonTrait)]
pub(crate) enum WireRustGenerator<'a> {
    Boxed(BoxedWireRustGenerator<'a>),
    DartOpaque(DartOpaqueWireRustGenerator<'a>),
    Delegate(DelegateWireRustGenerator<'a>),
    Dynamic(DynamicWireRustGenerator<'a>),
    EnumRef(EnumRefWireRustGenerator<'a>),
    GeneralList(GeneralListWireRustGenerator<'a>),
    Optional(OptionalWireRustGenerator<'a>),
    OptionalList(OptionalListWireRustGenerator<'a>),
    Primitive(PrimitiveWireRustGenerator<'a>),
    PrimitiveList(PrimitiveListWireRustGenerator<'a>),
    Record(RecordWireRustGenerator<'a>),
    RustOpaque(RustOpaqueWireRustGenerator<'a>),
    StructRef(StructRefWireRustGenerator<'a>),
    Unencodable(UnencodableWireRustGenerator<'a>),
}

codegen_generator_structs!(
    WireRustGenerator;

    Boxed,
    DartOpaque,
    Delegate,
    Dynamic,
    EnumRef,
    GeneralList,
    Optional,
    OptionalList,
    Primitive,
    PrimitiveList,
    Record,
    RustOpaque,
    StructRef,
    Unencodable,
);

#[derive(Debug, Clone)]
pub(crate) struct WireRustGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    // pub(crate) config: &'a GeneratorWireRustInternalConfig, // TODO
}
