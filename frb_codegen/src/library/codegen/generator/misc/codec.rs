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
    ($struct_name:ident) => (
        crate::codegen_codec_structs!(
            @private

            $struct_name;

            Cst,
            Dco,
            Sse,
        );
    );
    (@private $struct_name:ident ; $($name:ident),*,) => (
        paste::paste! {
            pub(crate) enum $struct_name {
                $(
                $name([<$name $struct_name>]),
                )*
            }

            impl $struct_name {
                pub(crate) fn new(mode: CodecMode) -> Self {
                    match mode {
                        $(
                        CodecMode::$name => Self::$name([<$name $struct_name>] {}),
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
