use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::future::MirTypeFuture;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::{
    Boxed, DartFn, DartOpaque, Delegate, Dynamic, EnumRef, Future, GeneralList, Optional,
    Primitive, PrimitiveList, Record, RustAutoOpaqueImplicit, RustOpaque, StructRef,
};
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use anyhow::{bail, ensure, Context};
use quote::ToTokens;
use syn::{GenericArgument, PathArguments, PathSegment, TypePath};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_future(
        &mut self,
        type_path: &TypePath,
        last_segment: &SplayedSegment,
        _splayed_segments: &[SplayedSegment],
    ) -> anyhow::Result<Option<MirType>> {
        Ok(Some(
            match (last_segment.name, last_segment.type_arguments().as_slice()) {
                ("DartFnFuture", [inner]) => {
                    let inner = self.parse_type(inner)?;

                    // This will stop the whole generator and tell the users, so we do not care about testing it
                    // frb-coverage:ignore-start
                    ensure!(
                        !matches!(inner, Future(_)),
                        "Nested futures without indirection are not supported. {}",
                        type_path.to_token_stream()
                    );
                    // frb-coverage:ignore-end

                    Future(match inner {
                        StructRef(..)
                        | EnumRef(..)
                        | RustAutoOpaqueImplicit(..)
                        | RustOpaque(..)
                        | DartOpaque(..)
                        | DartFn(..)
                        | Primitive(..)
                        | Record(..)
                        | Optional(..)
                        | Delegate(MirTypeDelegate::PrimitiveEnum(..)) => {
                            MirTypeFuture::new_with_boxed_wrapper(inner.clone())
                        }
                        Delegate(MirTypeDelegate::Time(..)) => {
                            MirTypeFuture::new_with_boxed_wrapper(inner.clone())
                        }
                        PrimitiveList(_) | GeneralList(_) | Boxed(_) | Dynamic(_) | Delegate(_) => {
                            MirTypeFuture::new(inner.clone())
                        }
                        // frb-coverage:ignore-start
                        // TODO (@vhdirk): Is this really unreachable?
                        Future(_) | MirType::TraitDef(_) => unreachable!(),
                        // frb-coverage:ignore-end
                    })
                }
                ("Pin", [inner]) => match self.parse_type(&inner)? {
                    Boxed(MirTypeBoxed { inner, .. }) => match *inner {
                        MirType::Delegate(MirTypeDelegate::Future(future)) => {
                            Future(MirTypeFuture {
                                output: future.inner.clone(),
                            })
                        }
                        _ => return Ok(None),
                    },
                    _ => return Ok(None),
                },

                _ => return Ok(None),
            },
        ))
    }

    pub(crate) fn parse_type_impl_trait_future(
        &mut self,
        name: &str,
        segment: &PathSegment,
    ) -> anyhow::Result<Option<MirType>> {
        if let PathArguments::AngleBracketed(arguments) = &segment.arguments {
            // Since Fn/FnOnce are handled by `Parenthesized`, here we can assume the bracketed type is indeed a `Future`, and
            // we can safely grab the `Output=` associated type. There should be only one.

            let output = match arguments.args.first() {
                Some(GenericArgument::AssocType(assoc_ty)) => self
                    .parse_type(&assoc_ty.ty)
                    .context("cannot parse future output type"),
                _ => bail!("Unknown type args for: {name}"),
            }?;

            return Ok(Some(MirType::Future(MirTypeFuture {
                output: Box::new(output),
            })));
        }

        Ok(None)
    }
}
