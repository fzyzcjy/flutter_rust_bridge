use crate::codegen::ir::ty::IrType;
use serde::Serialize;
use strum_macros::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, Display)]
pub(crate) enum CodecMode {
    Cst,
    Dco,
    Sse,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Hash)]
pub(crate) struct CodecModePack {
    pub dart2rust: CodecMode,
    pub rust2dart: CodecMode,
}

#[doc(hidden)]
#[macro_export]
macro_rules! codegen_codec_structs {
    ($enum_name:ident) => (
        crate::codegen_codec_structs!(
            @private

            $enum_name;

            Cst,
            Dco,
            Sse,
        );
    );
    (@private $enum_name:ident ; $($name:ident),*,) => (
        paste::paste! {
            #[enum_dispatch(BaseCodecEntrypointTrait<WireDartGeneratorContext<'a>, Box<dyn WireDartCodecOutputSpec>>)]
            #[enum_dispatch([<$enum_name Trait>])]
            pub(crate) enum $enum_name<'a> {
                $(
                $name([<$name $enum_name>]),
                )*
            }

            impl<'a> $enum_name<'a> {
                pub(crate) fn new(mode: CodecMode) -> Self {
                    match mode {
                        $(
                        CodecMode::$name => Self::$name([<$name $enum_name>] {}),
                        )*
                    }
                }
            }
        }
    )
}

pub(crate) trait BaseCodecEntrypointTrait<C, O> {
    fn generate_encode(&self, context: C, types: &[IrType]) -> Option<O>;

    fn generate_decode(&self, context: C, types: &[IrType]) -> Option<O>;
}
