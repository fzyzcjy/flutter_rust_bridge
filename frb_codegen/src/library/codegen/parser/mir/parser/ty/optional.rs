use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
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

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
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

                Optional(match inner {
                    StructRef(..)
                    | EnumRef(..)
                    | RustAutoOpaqueImplicit(..)
                    | RustOpaque(..)
                    | DartOpaque(..)
                    | DartFn(..)
                    | Primitive(..)
                    | Record(..)
                    | Delegate(MirTypeDelegate::PrimitiveEnum(..)) => {
                        MirTypeOptional::new_with_boxed_wrapper(inner.clone())
                    }
                    Delegate(MirTypeDelegate::Time(..)) => {
                        MirTypeOptional::new_with_boxed_wrapper(inner.clone())
                    }
                    PrimitiveList(_) | GeneralList(_) | Boxed(_) | Dynamic(_) | Delegate(_) => {
                        MirTypeOptional::new(inner.clone())
                    }
                    // frb-coverage:ignore-start
                    Optional(_) | MirType::TraitDef(_) => unreachable!(),
                    // frb-coverage:ignore-end
                })
            }

            _ => return Ok(None),
        }))
    }
}
