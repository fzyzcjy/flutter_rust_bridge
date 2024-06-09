use crate::codegen::ir::mir::func::{MirFuncOwnerInfo, OwnershipMode};
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateProxyVariant};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::real::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::mir::parser::ty::result::parse_type_maybe_result;
use crate::codegen::parser::mir::parser::ty::TypeParserParsingContext;
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
            ReturnType::Type(_, ty) => remove_reference_type(remove_primitive_unit(
                self.parse_fn_output_type(ty, owner, context, attributes)?,
            )),
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

fn remove_reference_type(info: FunctionPartialInfo) -> FunctionPartialInfo {
    if let Some(MirType::RustAutoOpaqueImplicit(MirTypeRustAutoOpaqueImplicit {
        ownership_mode,
        ..
    })) = &info.ok_output
    {
        if *ownership_mode != OwnershipMode::Owned {
            log::debug!("remove_reference_type: detect output type is a reference, thus set to unit (info={:?})", info);
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

fn parse_proxy_return_type(mir: MirType, owner: &MirFuncOwnerInfo) -> anyhow::Result<MirType> {
    if let MirType::RustAutoOpaqueImplicit(mir_inner) = &mir {
        if mir_inner.ownership_mode == OwnershipMode::Ref
            || mir_inner.ownership_mode == OwnershipMode::RefMut
        {
            if let MirFuncOwnerInfo::Method(method) = owner {
                return Ok(MirType::Delegate(MirTypeDelegate::ProxyVariant(
                    MirTypeDelegateProxyVariant {
                        inner: Box::new(mir),
                        upstream: Box::new(method.owner_ty.clone()),
                    },
                )));
            }
        }
    }
    bail!("This return type is not currently compatible with `#[frb(proxy)]` yet")
}
