#[macro_export]
macro_rules! codegen_generator_structs {
    ($generator_name:ident; $(#[$attribute:meta])*; $($name:ident),*,) => (
        paste! {
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
