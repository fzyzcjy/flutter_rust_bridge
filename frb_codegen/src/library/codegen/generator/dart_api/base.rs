use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(IrTypeTrait)]
pub enum DartApiGenerator<'a> {
    // alphabetical order
    Boxed(BoxedDartApiGenerator<'a>),
    DartOpaque(DartOpaqueDartApiGenerator<'a>),
    Delegate(DelegateDartApiGenerator<'a>),
    Dynamic(DynamicDartApiGenerator<'a>),
    EnumRef(EnumRefDartApiGenerator<'a>),
    GeneralList(GeneralListDartApiGenerator<'a>),
    Optional(OptionalDartApiGenerator<'a>),
    OptionalList(OptionalListDartApiGenerator<'a>),
    Primitive(PrimitiveDartApiGenerator<'a>),
    PrimitiveList(PrimitiveListDartApiGenerator<'a>),
    Record(RecordDartApiGenerator<'a>),
    RustOpaque(RustOpaqueDartApiGenerator<'a>),
    StructRef(StructRefDartApiGenerator<'a>),
    Unencodable(UnencodableDartApiGenerator<'a>),
}

#[derive(Debug, Clone)]
struct DartApiGeneratorContext<'a> {
    ir_pack: &'a IrPack,
}

impl<'a> DartApiGenerator<'a> {
    pub(crate) fn new(ty: IrType, ir_pack: &'a IrPack) -> Option<Self> {
        let context = DartApiGeneratorContext { ir_pack };
        Some(match ty {
            Boxed(ir) => BoxedDartApiGenerator::new(ir, context).into(),
            DartOpaque(ir) => DartOpaqueDartApiGenerator::new(ir, context).into(),
            Delegate(ir) => DelegateDartApiGenerator::new(ir, context).into(),
            Dynamic(ir) => DynamicDartApiGenerator::new(ir, context).into(),
            EnumRef(ir) => EnumRefDartApiGenerator::new(ir, context).into(),
            GeneralList(ir) => GeneralListDartApiGenerator::new(ir, context).into(),
            Optional(ir) => OptionalDartApiGenerator::new(ir, context).into(),
            OptionalList(ir) => OptionalListDartApiGenerator::new(ir, context).into(),
            TODO => TODO,
            _ => return None,
        })
    }
}

macro_rules! dart_api_generator_struct {
    ($cls:ident, $ir_cls:ty) => {
        #[derive(Debug, Clone)]
        pub(super) struct $cls<'a> {
            ir: $ir_cls,
            ir_pack: &'a crate::codegen::ir::pack::IrPack,
        }

        impl<'a> $cls<'a> {
            pub(super) fn new(ir: $ir_cls, context: DartApiGeneratorContext<'a>) -> Self {
                Self { ir, ir_pack }
            }
        }
    };
}

dart_api_generator_struct!(BoxedDartApiGenerator, IrTypeBoxed);
dart_api_generator_struct!(DartOpaqueDartApiGenerator, IrTypeDartOpaque);
dart_api_generator_struct!(DelegateDartApiGenerator, IrTypeDelegate);
dart_api_generator_struct!(DynamicDartApiGenerator, IrTypeDynamic);
dart_api_generator_struct!(EnumRefDartApiGenerator, IrTypeEnumRef);
dart_api_generator_struct!(GeneralListDartApiGenerator, IrTypeGeneralList);
dart_api_generator_struct!(OptionalDartApiGenerator, IrTypeOptional);
dart_api_generator_struct!(OptionalListDartApiGenerator, IrTypeOptionalList);
dart_api_generator_struct!(PrimitiveDartApiGenerator, IrTypePrimitive);
dart_api_generator_struct!(PrimitiveListDartApiGenerator, IrTypePrimitiveList);
dart_api_generator_struct!(RecordDartApiGenerator, IrTypeRecord);
dart_api_generator_struct!(RustOpaqueDartApiGenerator, IrTypeRustOpaque);
dart_api_generator_struct!(StructRefDartApiGenerator, IrTypeStructRef);
dart_api_generator_struct!(UnencodableDartApiGenerator, IrTypeUnencodable);
