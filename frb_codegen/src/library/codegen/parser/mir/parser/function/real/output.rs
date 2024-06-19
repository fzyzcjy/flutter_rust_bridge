use crate::codegen::ir::mir::func::{MirFuncOwnerInfo, OwnershipMode};
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateProxyVariant};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::real::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::mir::parser::ty::result::parse_type_maybe_result;
use crate::codegen::parser::mir::parser::ty::TypeParserParsingContext;
use crate::codegen::parser::mir::ParseMode;
use anyhow::bail;
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_output(
        &mut self,
        sig: &Signature,
        owner: &MirFuncOwnerInfo,
        context: &TypeParserParsingContext,
        attributes: &FrbAttributes,
    ) -> anyhow::Result<FunctionPartialInfo> {
        Ok(match &sig.output {
            ReturnType::Type(_, ty) => remove_reference_type(
                remove_primitive_unit(self.parse_fn_output_type(ty, owner, context, attributes)?),
                context.parse_mode,
                &sig.ident.to_string(),
            ),
            ReturnType::Default => Default::default(),
        })
    }

    #[allow(clippy::single_match)] // deliberate do so to ensure style consistency
    fn parse_fn_output_type(
        &mut self,
        ty: &Type,
        owner: &MirFuncOwnerInfo,
        context: &TypeParserParsingContext,
        attributes: &FrbAttributes,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let mir = self.type_parser.parse_type(ty, context)?;
        let mir = parse_maybe_proxy_return_type(mir, owner, attributes)?;
        let info = parse_type_maybe_result(&mir, self.type_parser, context)?;
        Ok(FunctionPartialInfo {
            ok_output: Some(info.ok_output),
            error_output: info.error_output,
            ..Default::default()
        })
    }
}

// Convert primitive Unit type -> None
fn remove_primitive_unit(info: FunctionPartialInfo) -> FunctionPartialInfo {
    if info.ok_output == Some(MirType::Primitive(MirTypePrimitive::Unit)) {
        return FunctionPartialInfo {
            ok_output: None,
            ..info
        };
    }
    info
}

fn remove_reference_type(
    info: FunctionPartialInfo,
    debug_parse_mode: ParseMode,
    debug_function_name: &str,
) -> FunctionPartialInfo {
    if let Some(MirType::RustAutoOpaqueImplicit(MirTypeRustAutoOpaqueImplicit {
        ownership_mode,
        ..
    })) = &info.ok_output
    {
        if *ownership_mode != OwnershipMode::Owned {
            if debug_parse_mode != ParseMode::Early {
                log::info!(
                    "Output type of `{debug_function_name}` is a reference, thus currently set to unit type. \
                    The \"lifetimes\" section in doc may be interesting."
                );
            }
            return FunctionPartialInfo {
                ok_output: None,
                ..info
            };
        }
    }
    info
}

fn parse_maybe_proxy_return_type(
    mir: MirType,
    owner: &MirFuncOwnerInfo,
    attributes: &FrbAttributes,
) -> anyhow::Result<MirType> {
    if attributes.proxy() {
        parse_proxy_return_type(mir, owner)
    } else {
        Ok(mir)
    }
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn parse_proxy_return_type(mir: MirType, owner: &MirFuncOwnerInfo) -> anyhow::Result<MirType> {
    // frb-coverage:ignore-end
    if let MirType::RustAutoOpaqueImplicit(mir_inner) = &mir {
        if mir_inner.ownership_mode == OwnershipMode::Ref
            || mir_inner.ownership_mode == OwnershipMode::RefMut
        {
            if let MirFuncOwnerInfo::Method(method) = owner {
                return Ok(MirType::Delegate(MirTypeDelegate::ProxyVariant(
                    MirTypeDelegateProxyVariant {
                        inner: Box::new(mir),
                        upstream: Box::new(method.owner_ty.clone()),
                        upstream_method_name: method.actual_method_name.clone(),
                    },
                )));
            }
        }
    }
    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    bail!("This return type is not currently compatible with `#[frb(proxy)]` yet")
    // frb-coverage:ignore-end
}
