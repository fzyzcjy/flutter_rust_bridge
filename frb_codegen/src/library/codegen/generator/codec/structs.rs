use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::codegen::ir::mir::ty::MirType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Display, EnumIter, EnumString,
)]
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
            pub(crate) struct [<Wire $partial_name CodecEntrypoint>]<'a>(
                Box<dyn [<Wire $partial_name CodecEntrypointTrait>]<'a>>
            );

            impl<'a> From<CodecMode> for [<Wire $partial_name CodecEntrypoint>]<'a> {
                fn from(mode: CodecMode) -> Self {
                    match mode {
                        $(
                        CodecMode::$name => Self(Box::new([<$name Wire $partial_name CodecEntrypoint>] {})),
                        )*
                    }
                }
            }

            impl<'a> std::ops::Deref for [<Wire $partial_name CodecEntrypoint>]<'a> {
                type Target = Box<dyn [<Wire $partial_name CodecEntrypointTrait>]<'a>>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl<'a> [<Wire $partial_name CodecEntrypoint>]<'a> {
                pub(crate) fn generate_all(
                    context: [<Wire $partial_name GeneratorContext>],
                    cache: &MirPackComputedCache,
                    mode: EncodeOrDecode,
                ) -> [<Wire $partial_name CodecOutputSpec>] {
                    CodecMode::iter()
                        .flat_map(|codec| [<Wire $partial_name CodecEntrypoint>]::from(codec)
                            .generate(context, &get_interest_types_for_codec(cache, codec), mode))
                        .collect()
                }
            }

            #[derive(Clone, Serialize)]
            pub(crate) struct [<Wire $partial_name CodecOutputSpec>] {
                pub(crate) inner: Acc<Vec<[<Wire $partial_name OutputCode>]>>,
            }

            impl std::iter::FromIterator<[<Wire $partial_name CodecOutputSpec>]> for [<Wire $partial_name CodecOutputSpec>] {
                fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
                    Self {
                        inner: iter.into_iter().map(|x| x.inner).collect(),
                    }
                }
            }
        }
    )
}

pub(crate) fn get_interest_types_for_codec(
    cache: &MirPackComputedCache,
    codec: CodecMode,
) -> Vec<MirType> {
    match codec {
        CodecMode::Cst => cache.distinct_types_for_codec[&codec].clone(),
        // Consider all types in Rust, since users may want IntoDart and IntoIntoDart for DartDynamic etc
        // And all types in Dart, since DartFn needs DCO
        CodecMode::Dco => cache.distinct_types.clone(),
        // For simplicity, consider all types, since (1) PDE needs SSE (2) non-SSE DartFn still requires SSE
        CodecMode::Sse => cache.distinct_types.clone(),
        CodecMode::Pde => vec![],
    }
}

pub(crate) trait BaseCodecEntrypointTrait<C, O> {
    fn generate(&self, context: C, types: &[MirType], mode: EncodeOrDecode) -> Option<O>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum EncodeOrDecode {
    Encode,
    Decode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use std::collections::HashMap;

    /// Delegates only PDE to SSE while retaining every other codec mode.
    #[test]
    fn delegates_only_pde_codec_mode() {
        assert_eq!(CodecMode::Pde.delegate(), Some(CodecMode::Sse));
        assert_eq!(CodecMode::Cst.delegate(), None);
        assert_eq!(CodecMode::Dco.delegate_or_self(), CodecMode::Dco);
        assert_eq!(CodecMode::Pde.delegate_or_self(), CodecMode::Sse);
    }

    /// Lists Dart-to-Rust and Rust-to-Dart modes in their configured order.
    #[test]
    fn lists_both_configured_codec_modes() {
        let pack = CodecModePack {
            dart2rust: CodecMode::Cst,
            rust2dart: CodecMode::Dco,
        };

        assert_eq!(pack.all(), vec![CodecMode::Cst, CodecMode::Dco]);
    }

    /// Selects cached or global interest types for all codec modes.
    #[test]
    fn selects_interest_types_for_every_codec_mode() {
        let cst_types = vec![MirType::Primitive(MirTypePrimitive::U8)];
        let global_types = vec![
            MirType::Primitive(MirTypePrimitive::I32),
            MirType::Primitive(MirTypePrimitive::F64),
        ];
        let cache = MirPackComputedCache {
            distinct_types: global_types.clone(),
            distinct_types_for_codec: HashMap::from([
                (CodecMode::Cst, cst_types.clone()),
                (
                    CodecMode::Dco,
                    vec![MirType::Primitive(MirTypePrimitive::I8)],
                ),
                (
                    CodecMode::Sse,
                    vec![MirType::Primitive(MirTypePrimitive::U16)],
                ),
                (
                    CodecMode::Pde,
                    vec![MirType::Primitive(MirTypePrimitive::U32)],
                ),
            ]),
        };

        let cases = [
            (CodecMode::Cst, cst_types),
            (CodecMode::Dco, global_types.clone()),
            (CodecMode::Sse, global_types),
            (CodecMode::Pde, vec![]),
        ];

        for (codec, expected) in cases {
            assert_eq!(get_interest_types_for_codec(&cache, codec), expected);
        }
    }
}
