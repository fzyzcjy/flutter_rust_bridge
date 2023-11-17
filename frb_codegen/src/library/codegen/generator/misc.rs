use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs {
    ($generator_name:ident;$($name:ident),*,) => (
        paste! {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Target {
    Io,
    Wasm,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TargetOrCommon {
    Common,
    Io,
    Wasm,
}

/// In WASM, these types belong to the JS scope-local heap, **NOT** the Rust heap and
/// therefore do not implement [Send]. More specifically, these are types wasm-bindgen
/// can't handle yet.
pub fn is_js_value(ty: &IrType) -> bool {
    match ty {
        IrType::GeneralList(_)
        | IrType::OptionalList(_)
        | IrType::StructRef(_)
        | IrType::EnumRef(_)
        | IrType::RustOpaque(_)
        | IrType::DartOpaque(_)
        | IrType::Record(_) => true,
        IrType::Boxed(IrTypeBoxed { inner, .. }) => is_js_value(inner),
        IrType::Delegate(inner) => is_js_value(&inner.get_delegate()),
        IrType::Optional(inner) => is_js_value(&inner.inner),
        IrType::Primitive(_) | IrType::PrimitiveList(_) => false,
        IrType::Dynamic(_) | IrType::Unencodable(_) => unreachable!(),
    }
}
