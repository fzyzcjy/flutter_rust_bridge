use crate::codegen::ir::mir::ty::optional::MirTypeOptional;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{
    Boxed, DartFn, DartOpaque, Delegate, Dynamic, EnumRef, GeneralList, Optional, Primitive,
    PrimitiveList, Record, RustAutoOpaqueImplicit, RustOpaque, StructRef,
};
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use anyhow::ensure;
use quote::ToTokens;
use syn::TypePath;

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type_path_data_optional(
        &mut self,
        type_path: &TypePath,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        Ok(Some(match last_segment {
            ("Option", [inner]) => {
                let inner = self.parse_type(inner)?;

                // This will stop the whole generator and tell the users, so we do not care about testing it
                // frb-coverage:ignore-start
                ensure!(
                    !matches!(inner, Optional(_)),
                    "Nested optionals without indirection are not supported. {}",
                    type_path.to_token_stream()
                );
                // frb-coverage:ignore-end

                Optional(if optional_inner_needs_boxed_wrapper(&inner) {
                    MirTypeOptional::new_with_boxed_wrapper(inner)
                } else {
                    match inner {
                        PrimitiveList(_)
                        | GeneralList(_)
                        | Boxed(_)
                        | Dynamic(_)
                        | Delegate(_) => MirTypeOptional::new(inner),
                        // frb-coverage:ignore-start
                        Optional(_) | MirType::TraitDef(_) => unreachable!(),
                        // frb-coverage:ignore-end
                        StructRef(..)
                        | EnumRef(..)
                        | RustAutoOpaqueImplicit(..)
                        | RustOpaque(..)
                        | DartOpaque(..)
                        | DartFn(..)
                        | Primitive(..)
                        | Record(..) => unreachable!(),
                    }
                })
            }

            _ => return Ok(None),
        }))
    }
}

fn optional_inner_needs_boxed_wrapper(inner: &MirType) -> bool {
    inner.is_primitive()
        || matches!(
            inner,
            StructRef(..)
            | EnumRef(..)
            | RustAutoOpaqueImplicit(..)
            | RustOpaque(..)
            | DartOpaque(..)
            | DartFn(..)
            | Record(..)
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::ty::delegate::{
        MirTypeDelegate, MirTypeDelegateCastedPrimitive,
    };
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

    /// Ensures casted primitives use the production optional boxing classifier.
    #[test]
    fn casted_primitive_optional_uses_boxed_wrapper() {
        let inner = Delegate(MirTypeDelegate::CastedPrimitive(
            MirTypeDelegateCastedPrimitive {
                inner: MirTypePrimitive::I64,
            },
        ));

        assert!(optional_inner_needs_boxed_wrapper(&inner));
    }
}
