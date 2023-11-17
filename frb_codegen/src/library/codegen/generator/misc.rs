#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs {
    ($($enum_dispatch_name:ident),*;$generator_name:ident;$($name:ident),*,) => (
        paste! {
            $(
                #[enum_dispatch($enum_dispatch_name)]
            )*
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

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Common,
    Io,
    Wasm,
}

impl Target {
    #[inline]
    pub const fn is_wasm(&self) -> bool {
        matches!(self, Self::Wasm)
    }
}
