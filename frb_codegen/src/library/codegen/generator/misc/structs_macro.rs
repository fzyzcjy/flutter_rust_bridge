#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs {
    ($(#[$attribute:meta])* $generator_name:ident) => (
        $crate::codegen_generator_structs!(
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
            Primitive,
            PrimitiveList,
            Record,
            RustAutoOpaqueImplicit,
            RustOpaque,
            StructRef,
        );
    );
    (@private $(#[$attribute:meta])* $generator_name:ident ; $($name:ident),*,) => (
        use $crate::codegen::mir::pack::IrPack;
        use $crate::codegen::mir::ty::boxed::IrTypeBoxed;
        use $crate::codegen::mir::ty::dart_fn::IrTypeDartFn;
        use $crate::codegen::mir::ty::dart_opaque::IrTypeDartOpaque;
        use $crate::codegen::mir::ty::delegate::IrTypeDelegate;
        use $crate::codegen::mir::ty::dynamic::IrTypeDynamic;
        use $crate::codegen::mir::ty::enumeration::IrTypeEnumRef;
        use $crate::codegen::mir::ty::general_list::IrTypeGeneralList;
        use $crate::codegen::mir::ty::optional::IrTypeOptional;
        use $crate::codegen::mir::ty::primitive::IrTypePrimitive;
        use $crate::codegen::mir::ty::primitive_list::IrTypePrimitiveList;
        use $crate::codegen::mir::ty::record::IrTypeRecord;
        use $crate::codegen::mir::ty::rust_auto_opaque_implicit::IrTypeRustAutoOpaqueImplicit;
        use $crate::codegen::mir::ty::rust_opaque::IrTypeRustOpaque;
        use $crate::codegen::mir::ty::structure::IrTypeStructRef;
        use $crate::codegen::mir::ty::IrType;
        // cargo fix wrongly removes this import
        #[allow(unused_imports)]
        use $crate::codegen::mir::ty::IrType::*;


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
                // Because only some of them are used
                #[allow(dead_code)]
                pub(crate) fn new(ty: impl Into<IrType>, context: [<$generator_name Context>]<'a>) -> Self {
                    // This is surely used, seems to be bug of coverage tool
                    // frb-coverage:ignore-start
                    match ty.into() {
                        $(
                            $name(ir) => Self::$name([<$name $generator_name>]::new(ir, context)),
                        )*
                    }
                    // frb-coverage:ignore-end
                }
            }

            // This is surely used, seems to be bug of coverage tool
            // frb-coverage:ignore-start
            #[enum_dispatch]
            // frb-coverage:ignore-end
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
