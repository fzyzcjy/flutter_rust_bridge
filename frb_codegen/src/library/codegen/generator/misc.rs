use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::enum_map;
use anyhow::bail;
use std::convert::TryFrom;
#[doc(hidden)]
#[macro_export]
macro_rules! codegen_generator_structs {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum_macros::EnumIter)]
pub enum Target {
    Io,
    Wasm,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, strum_macros::EnumIter)]
pub enum TargetOrCommon {
    Common,
    Io,
    Wasm,
}

enum_map!(
    TargetOrCommonMap, TargetOrCommon;
    Common, Io, Wasm;
    common, io, wasm;
);

impl TryFrom<TargetOrCommon> for Target {
    type Error = anyhow::Error;

    fn try_from(src: TargetOrCommon) -> Result<Self, Self::Error> {
        Ok(match src {
            TargetOrCommon::Common => bail!("Cannot convert TargetOrCommon::Common to Target"),
            TargetOrCommon::Io => Target::Io,
            TargetOrCommon::Wasm => Target::Wasm,
        })
    }
}

impl From<Target> for TargetOrCommon {
    fn from(value: Target) -> Self {
        match value {
            Target::Io => TargetOrCommon::Io,
            Target::Wasm => TargetOrCommon::Wasm,
        }
    }
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
