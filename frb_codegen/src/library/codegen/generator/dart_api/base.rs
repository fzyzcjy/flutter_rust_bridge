use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::*;
use enum_dispatch::enum_dispatch;

macro_rules! primary {
    ($($x:expr),+) => (
        fn f() {
            g($($x),+);
        }
    )
}

primary!(
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

macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}

// #[enum_dispatch(IrTypeTrait)]
// pub enum DartApiGenerator<'a> {
//     // alphabetical order
//     Boxed(BoxedDartApiGenerator<'a>),
//     DartOpaque(DartOpaqueDartApiGenerator<'a>),
//     Delegate(DelegateDartApiGenerator<'a>),
//     Dynamic(DynamicDartApiGenerator<'a>),
//     EnumRef(EnumRefDartApiGenerator<'a>),
//     GeneralList(GeneralListDartApiGenerator<'a>),
//     Optional(OptionalDartApiGenerator<'a>),
//     OptionalList(OptionalListDartApiGenerator<'a>),
//     Primitive(PrimitiveDartApiGenerator<'a>),
//     PrimitiveList(PrimitiveListDartApiGenerator<'a>),
//     Record(RecordDartApiGenerator<'a>),
//     RustOpaque(RustOpaqueDartApiGenerator<'a>),
//     StructRef(StructRefDartApiGenerator<'a>),
//     Unencodable(UnencodableDartApiGenerator<'a>),
// }
//
// #[derive(Debug, Clone)]
// struct DartApiGeneratorContext<'a> {
//     ir_pack: &'a IrPack,
// }
//
// impl<'a> DartApiGenerator<'a> {
//     pub(crate) fn new(ty: IrType, ir_pack: &'a IrPack) -> Option<Self> {
//         let context = DartApiGeneratorContext { ir_pack };
//         Some(match ty {
//             Boxed(ir) => BoxedDartApiGenerator::new(ir, context).into(),
//             DartOpaque(ir) => DartOpaqueDartApiGenerator::new(ir, context).into(),
//             Delegate(ir) => DelegateDartApiGenerator::new(ir, context).into(),
//             Dynamic(ir) => DynamicDartApiGenerator::new(ir, context).into(),
//             EnumRef(ir) => EnumRefDartApiGenerator::new(ir, context).into(),
//             GeneralList(ir) => GeneralListDartApiGenerator::new(ir, context).into(),
//             Optional(ir) => OptionalDartApiGenerator::new(ir, context).into(),
//             OptionalList(ir) => OptionalListDartApiGenerator::new(ir, context).into(),
//             TODO => TODO,
//             _ => return None,
//         })
//     }
// }
//
// macro_rules! dart_api_generator_struct {
//     ($cls:ident, $ir_cls:ty) => {
//         #[derive(Debug, Clone)]
//         pub(super) struct $cls<'a> {
//             ir: $ir_cls,
//             ir_pack: &'a crate::codegen::ir::pack::IrPack,
//         }
//
//         impl<'a> $cls<'a> {
//             pub(super) fn new(ir: $ir_cls, context: DartApiGeneratorContext<'a>) -> Self {
//                 Self { ir, ir_pack }
//             }
//         }
//     };
// }
//
// dart_api_generator_struct!(BoxedDartApiGenerator, IrTypeBoxed);
// dart_api_generator_struct!(DartOpaqueDartApiGenerator, IrTypeDartOpaque);
// dart_api_generator_struct!(DelegateDartApiGenerator, IrTypeDelegate);
// dart_api_generator_struct!(DynamicDartApiGenerator, IrTypeDynamic);
// dart_api_generator_struct!(EnumRefDartApiGenerator, IrTypeEnumRef);
// dart_api_generator_struct!(GeneralListDartApiGenerator, IrTypeGeneralList);
// dart_api_generator_struct!(OptionalDartApiGenerator, IrTypeOptional);
// dart_api_generator_struct!(OptionalListDartApiGenerator, IrTypeOptionalList);
// dart_api_generator_struct!(PrimitiveDartApiGenerator, IrTypePrimitive);
// dart_api_generator_struct!(PrimitiveListDartApiGenerator, IrTypePrimitiveList);
// dart_api_generator_struct!(RecordDartApiGenerator, IrTypeRecord);
// dart_api_generator_struct!(RustOpaqueDartApiGenerator, IrTypeRustOpaque);
// dart_api_generator_struct!(StructRefDartApiGenerator, IrTypeStructRef);
// dart_api_generator_struct!(UnencodableDartApiGenerator, IrTypeUnencodable);
