use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
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
use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::*;
use crate::codegen_generator_structs;
use enum_dispatch::enum_dispatch;
use paste::paste;

#[enum_dispatch(ApiDartGeneratorInfoTrait)]
#[enum_dispatch(ApiDartGeneratorClassTrait)]
pub(crate) enum ApiDartGenerator<'a> {
    Boxed(BoxedApiDartGenerator<'a>),
    DartOpaque(DartOpaqueApiDartGenerator<'a>),
    Delegate(DelegateApiDartGenerator<'a>),
    Dynamic(DynamicApiDartGenerator<'a>),
    EnumRef(EnumRefApiDartGenerator<'a>),
    GeneralList(GeneralListApiDartGenerator<'a>),
    Optional(OptionalApiDartGenerator<'a>),
    OptionalList(OptionalListApiDartGenerator<'a>),
    Primitive(PrimitiveApiDartGenerator<'a>),
    PrimitiveList(PrimitiveListApiDartGenerator<'a>),
    Record(RecordApiDartGenerator<'a>),
    RustAutoOpaque(RustAutoOpaqueApiDartGenerator<'a>),
    RustOpaque(RustOpaqueApiDartGenerator<'a>),
    StructRef(StructRefApiDartGenerator<'a>),
    Unencodable(UnencodableApiDartGenerator<'a>),
}

codegen_generator_structs!(
    ApiDartGenerator;

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
    RustAutoOpaque,
    RustOpaque,
    StructRef,
    Unencodable,
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ApiDartGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorApiDartInternalConfig,
}
