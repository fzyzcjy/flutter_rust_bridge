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
            TraitDef,
        );
    );
    (@private $(#[$attribute:meta])* $generator_name:ident ; $($name:ident),*,) => (
        use $crate::codegen::ir::mir::pack::MirPack;
        use $crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
        use $crate::codegen::ir::mir::ty::dart_fn::MirTypeDartFn;
        use $crate::codegen::ir::mir::ty::dart_opaque::MirTypeDartOpaque;
        use $crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
        use $crate::codegen::ir::mir::ty::dynamic::MirTypeDynamic;
        use $crate::codegen::ir::mir::ty::enumeration::MirTypeEnumRef;
        use $crate::codegen::ir::mir::ty::general_list::MirTypeGeneralList;
        use $crate::codegen::ir::mir::ty::optional::MirTypeOptional;
        use $crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
        use $crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;
        use $crate::codegen::ir::mir::ty::record::MirTypeRecord;
        use $crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
        use $crate::codegen::ir::mir::ty::rust_opaque::MirTypeRustOpaque;
        use $crate::codegen::ir::mir::ty::structure::MirTypeStructRef;
        use $crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
        use $crate::codegen::ir::mir::ty::MirType;
        // cargo fix wrongly removes this import
        #[allow(unused_imports)]
        use $crate::codegen::ir::mir::ty::MirType::*;


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
                pub(crate) fn new(ty: impl Into<MirType>, context: [<$generator_name Context>]<'a>) -> Self {
                    // This is surely used, seems to be bug of coverage tool
                    // frb-coverage:ignore-start
                    match ty.into() {
                        $(
                            $name(mir) => Self::$name([<$name $generator_name>]::new(mir, context)),
                        )*
                    }
                    // frb-coverage:ignore-end
                }
            }

            // This is surely used, seems to be bug of coverage tool
            // frb-coverage:ignore-start
            #[enum_dispatch]
            // frb-coverage:ignore-end
            #[allow(dead_code)]
            pub(crate) trait [<$generator_name ImplTrait>] {
                fn mir_type(&self) -> MirType;
                fn context(&self) -> [<$generator_name Context>];
            }

            $(
                #[derive(Debug, Clone)]
                pub(crate) struct [<$name $generator_name>]<'a> {
                    pub(crate) mir: [<MirType $name>],
                    pub(crate) context: [<$generator_name Context>]<'a>,
                }

                impl<'a> [<$name $generator_name>]<'a> {
                    pub(crate) fn new(mir: [<MirType $name>], context: [<$generator_name Context>]<'a>) -> Self {
                        Self { mir, context }
                    }
                }

                impl<'a> [<$generator_name ImplTrait>] for [<$name $generator_name>]<'a> {
                    fn mir_type(&self) -> MirType { self.mir.clone().into() }
                    fn context(&self) -> [<$generator_name Context>] { self.context }
                }
            )*
        }
    )
}
