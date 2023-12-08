use crate::codegen::ir::ty::IrType;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, Display, EnumIter)]
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
            pub(crate) struct $struct_name<'a>(
                Box<dyn [<$struct_name Trait>]<'a>>
            );

            impl<'a> From<CodecMode> for $struct_name<'a> {
                fn from(mode: CodecMode) -> Self {
                    match mode {
                        $(
                        CodecMode::$name => Self(Box::new([<$name $struct_name>] {})),
                        )*
                    }
                }
            }

            impl<'a> std::ops::Deref for $struct_name<'a> {
                type Target = Box<dyn [<$struct_name Trait>]<'a>>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    )
}

pub(crate) trait BaseCodecEntrypointTrait<C, O> {
    fn generate_encode(&self, context: C, types: &[IrType]) -> Option<O>;

    fn generate_decode(&self, context: C, types: &[IrType]) -> Option<O>;
}

pub(crate) fn generate_codec_encode_decode<Codec, C, O>()
where
    Codec: From<CodecMode>,
    Codec: BaseCodecEntrypointTrait<C, O>,
{
    for codec_mode in CodecMode::iter() {
        let codec: Codec = codec_mode.into();
        codec.generate_encode(context, types);
        codec.generate_decode(context, types);
        todo!()
    }
}
