use crate::codegen::ir::ty::IrType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Display, EnumIter)]
pub(crate) enum CodecMode {
    Cst,
    Dco,
    Sse,
    Pde,
}

impl CodecMode {
    pub(crate) fn delegate(self) -> Option<Self> {
        match self {
            CodecMode::Pde => Some(CodecMode::Sse),
            _ => None,
        }
    }

    pub(crate) fn delegate_or_self(self) -> Self {
        self.delegate().unwrap_or(self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub(crate) struct CodecModePack {
    pub dart2rust: CodecMode,
    pub rust2dart: CodecMode,
}

impl CodecModePack {
    pub(crate) fn all(&self) -> Vec<CodecMode> {
        vec![self.dart2rust, self.rust2dart]
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! codegen_codec_structs {
    ($partial_name:ident) => (
        $crate::codegen_codec_structs!(
            @private

            $partial_name;

            Cst,
            Dco,
            Sse,
            Pde,
        );
    );
    (@private $partial_name:ident ; $($name:ident),*,) => (
        paste::paste! {
            pub(crate) struct [<$partial_name CodecEntrypoint>]<'a>(
                Box<dyn [<$partial_name CodecEntrypointTrait>]<'a>>
            );

            impl<'a> From<CodecMode> for [<$partial_name CodecEntrypoint>]<'a> {
                fn from(mode: CodecMode) -> Self {
                    match mode {
                        $(
                        CodecMode::$name => Self(Box::new([<$name $partial_name CodecEntrypoint>] {})),
                        )*
                    }
                }
            }

            impl<'a> std::ops::Deref for [<$partial_name CodecEntrypoint>]<'a> {
                type Target = Box<dyn [<$partial_name CodecEntrypointTrait>]<'a>>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<'a> [<$partial_name CodecEntrypoint>]<'a> {
                pub(crate) fn generate_all(
                    context: [<$partial_name GeneratorContext>],
                    cache: &IrPackComputedCache,
                    mode: EncodeOrDecode,
                ) -> [<$partial_name CodecOutputSpec>] {
                    CodecMode::iter()
                        .flat_map(|codec| [<$partial_name CodecEntrypoint>]::from(codec).generate(context, &cache.distinct_types_for_codec[&codec], mode))
                        .collect()
                }
            }

            #[derive(Clone, Serialize)]
            pub(crate) struct [<$partial_name CodecOutputSpec>] {
                pub(crate) inner: Acc<Vec<[<$partial_name OutputCode>]>>,
            }

            impl std::iter::FromIterator<[<$partial_name CodecOutputSpec>]> for [<$partial_name CodecOutputSpec>] {
                fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
                    Self {
                        inner: iter.into_iter().map(|x| x.inner).collect(),
                    }
                }
            }
        }
    )
}

pub(crate) trait BaseCodecEntrypointTrait<C, O> {
    fn generate(&self, context: C, types: &[IrType], mode: EncodeOrDecode) -> Option<O>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum EncodeOrDecode {
    Encode,
    Decode,
}
