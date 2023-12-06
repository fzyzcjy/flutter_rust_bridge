#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs_inner {
    ($generator_name:ident;$($name:ident),*,) => (
        paste! {
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

#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs_outer {
    ($generator_name:ident) => (
        codegen_generator_structs_inner!(
            $generator_name;

            Boxed,
            DartFn,
            DartOpaque,
            Delegate,
            Dynamic,
            EnumRef,
            GeneralList,
            Optional,
            OptionalList,
            Ownership,
            Primitive,
            PrimitiveList,
            Record,
            RustAutoOpaque,
            RustOpaque,
            StructRef,
            Unencodable,
        );
    )
}
