#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs {
    ($generator_name:ident;$($name:ident),*,) => (
        paste! {
            #[enum_dispatch(DartApiGeneratorDeclTrait)]
            #[enum_dispatch(DartApiGeneratorClassTrait)]
            pub(crate) enum $generator_name<'a> {
                $(
                    $name([<$name $generator_name>]<'a>),
                )*
            }

            impl<'a> $generator_name<'a> {
                pub(crate) fn new(ty: IrType, context: [<$generator_name Context>]<'a>) -> Self {
                    match ty {
                        $(
                            $name(ir) => Self::$name([<$name $generator_name>]::new(ir, context)),
                        )*
                    }
                }
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
            )*
        }
    )
}
