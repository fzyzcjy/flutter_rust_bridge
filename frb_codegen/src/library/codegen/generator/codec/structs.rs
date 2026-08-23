use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
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
        CodecMode::Pde => cache.distinct_types_for_codec[&codec]
            .iter()
            .filter(|ty| pde_web_direct_type(ty))
            .cloned()
            .collect(),
    }
}

pub(crate) fn pde_web_direct_codec(func: &MirFunc) -> bool {
    func.inputs
        .iter()
        .all(|input| pde_web_direct_type(&input.inner.ty))
        && pde_web_direct_type(&func.output.normal)
        && func.output.error.as_ref().map_or(true, pde_web_direct_type)
}

fn pde_web_direct_type(ty: &MirType) -> bool {
    match ty {
        MirType::Primitive(_) => true,
        MirType::PrimitiveList(list) => !matches!(
            list.primitive,
            MirTypePrimitive::Bool
                | MirTypePrimitive::Unit
                | MirTypePrimitive::Usize
                | MirTypePrimitive::Isize
        ),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
    use crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;

    /// Verifies that Web PDE direct calls exclude unsupported primitive lists.
    #[test]
    fn pde_web_direct_type_accepts_supported_types_only() {
        assert!(pde_web_direct_type(&MirType::Primitive(
            MirTypePrimitive::U8
        )));
        assert!(pde_web_direct_type(&MirType::PrimitiveList(
            MirTypePrimitiveList {
                primitive: MirTypePrimitive::U8,
                strict_dart_type: true,
            }
        )));
        for primitive in [
            MirTypePrimitive::Bool,
            MirTypePrimitive::Unit,
            MirTypePrimitive::Usize,
            MirTypePrimitive::Isize,
        ] {
            assert!(!pde_web_direct_type(&MirType::PrimitiveList(
                MirTypePrimitiveList {
                    primitive,
                    strict_dart_type: true,
                }
            )));
        }
        assert!(!pde_web_direct_type(&MirType::Delegate(
            MirTypeDelegate::String
        )));
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
