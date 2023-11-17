#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs {
    ($($name:ident),*,) => (
        paste! {
            #[enum_dispatch(DartApiGeneratorDeclTrait)]
            #[enum_dispatch(DartApiGeneratorClassTrait)]
            pub(crate) enum DartApiGenerator<'a> {
                $(
                    $name([<$name DartApiGenerator>]<'a>),
                )*
            }

            impl<'a> DartApiGenerator<'a> {
                pub(crate) fn new(ty: IrType, context: DartApiGeneratorContext<'a>) -> Self {
                    match ty {
                        $(
                            $name(ir) => Self::$name([<$name DartApiGenerator>]::new(ir, context)),
                        )*
                    }
                }
            }

            $(
                #[derive(Debug, Clone)]
                pub(crate) struct [<$name DartApiGenerator>]<'a> {
                    pub(crate) ir: [<IrType $name>],
                    pub(crate) context: DartApiGeneratorContext<'a>,
                }

                impl<'a> [<$name DartApiGenerator>]<'a> {
                    pub(crate) fn new(ir: [<IrType $name>], context: DartApiGeneratorContext<'a>) -> Self {
                        Self { ir, context }
                    }
                }
            )*
        }
    )
}
