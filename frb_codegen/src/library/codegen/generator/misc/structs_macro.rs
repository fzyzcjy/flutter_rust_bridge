#[macro_export]
macro_rules! codegen_generator_structs {
    ($(#[$attribute:meta])* $generator_name:ident) => (
        crate::codegen_generator_structs!(
            @private

            $(#[$attribute])*
            $generator_name;

            Boxed,
            DartFn,
            DartOpaque,
            Delegate,
            Dynamic,
            EnumRef,
            GeneralList,
            Optional,
            Ownership,
            Primitive,
            PrimitiveList,
            Record,
            RustAutoOpaque,
            RustOpaque,
            StructRef,
            Unencodable,
        );
    );
    (@private $(#[$attribute:meta])* $generator_name:ident ; $($name:ident),*,) => (
        use crate::codegen::ir::pack::IrPack;
        use crate::codegen::ir::ty::boxed::IrTypeBoxed;
        use crate::codegen::ir::ty::dart_fn::IrTypeDartFn;
        use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
        use crate::codegen::ir::ty::delegate::IrTypeDelegate;
        use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
        use crate::codegen::ir::ty::enumeration::IrTypeEnumRef;
        use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
        use crate::codegen::ir::ty::optional::IrTypeOptional;
        use crate::codegen::ir::ty::ownership::IrTypeOwnership;
        use crate::codegen::ir::ty::primitive::IrTypePrimitive;
        use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
        use crate::codegen::ir::ty::record::IrTypeRecord;
        use crate::codegen::ir::ty::rust_auto_opaque::IrTypeRustAutoOpaque;
        use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
        use crate::codegen::ir::ty::structure::IrTypeStructRef;
        use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
        use crate::codegen::ir::ty::IrType;
        use crate::codegen::ir::ty::IrType::*;

        paste::paste! {
            $(
            #[$attribute]
            )*
            pub(crate) enum $generator_name<'a> {
                $(
                    $name([<$name $generator_name>]<'a>),
                )*
            }

            impl<'a> $generator_name<'a> {
                pub(crate) fn new(ty: impl Into<IrType>, context: [<$generator_name Context>]<'a>) -> Self {
                    match ty.into() {
                        $(
                            $name(ir) => Self::$name([<$name $generator_name>]::new(ir, context)),
                        )*
                    }
                }
            }

            #[enum_dispatch]
            pub(crate) trait [<$generator_name ImplTrait>] {
                fn ir_type(&self) -> IrType;
                fn context(&self) -> [<$generator_name Context>];
            }

            $(
                #[derive(Debug, Clone)]
                pub(crate) struct [<$name $generator_name>]<'a> {
                    pub(crate) ir: [<IrType $name>],
                    pub(crate) context: [<$generator_name Context>]<'a>,
                }

                impl<'a> [<$name $generator_name>]<'a> {
                    pub(crate) fn new(ir: [<IrType $name>], context: [<$generator_name Context>]<'a>) -> Self {
                        Self { ir, context }
                    }
                }

                impl<'a> [<$generator_name ImplTrait>] for [<$name $generator_name>]<'a> {
                    fn ir_type(&self) -> IrType { self.ir.clone().into() }
                    fn context(&self) -> [<$generator_name Context>] { self.context }
                }
            )*
        }
    )
}
