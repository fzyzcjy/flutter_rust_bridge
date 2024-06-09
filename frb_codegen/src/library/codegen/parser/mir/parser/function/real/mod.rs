use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::function::HirFlatFunctionOwner;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncArgMode, MirFuncImplMode, MirFuncImplModeDartOnly,
    MirFuncInput, MirFuncMode, MirFuncOutput, MirFuncOwnerInfo, MirFuncOwnerInfoMethod,
    MirFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::mir::skip::MirSkipReason::IgnoredFunctionGeneric;
use crate::codegen::ir::mir::skip::{MirSkip, MirSkipReason};
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicitReason;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::func_or_skip::MirFuncOrSkip;
use crate::codegen::parser::mir::parser::ty::misc::parse_comments;
use crate::codegen::parser::mir::parser::ty::trait_def::parse_type_trait;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use crate::codegen::parser::mir::ParseMode;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::{Namespace, NamespacedName};
use anyhow::{bail, Context};
use itertools::{concat, Itertools};
use log::{debug, warn};
use std::fmt::Debug;
use syn::*;
use MirSkipReason::{IgnoredFunctionNotPub, IgnoredMisc};
use MirType::Primitive;

pub(crate) mod argument;
pub(crate) mod output;
mod transformer;

pub(crate) fn parse(
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
    config: &ParserMirInternalConfig,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    let mut function_parser = FunctionParser::new(type_parser);
    (src_fns.iter())
        // Sort to make things stable. The order of parsing functions will affect things like, e.g.,
        // which file an opaque type is put in.
        .sorted_by_key(|f| f.owner_and_name_for_dedup())
        .map(|f| {
            function_parser.parse_function(
                f,
                &config.force_codec_mode_pack,
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
                parse_mode,
                config.stop_on_error,
            )
        })
        .collect()
}

struct FunctionParser<'a, 'b> {
    type_parser: &'a mut TypeParser<'b>,
}

impl<'a, 'b> FunctionParser<'a, 'b> {
    fn new(type_parser: &'a mut TypeParser<'b>) -> Self {
        Self { type_parser }
    }

    #[allow(clippy::too_many_arguments)]
    fn parse_function(
        &mut self,
        func: &HirFlatFunction,
        force_codec_mode_pack: &Option<CodecModePack>,
        default_stream_sink_codec: CodecMode,
        default_rust_opaque_codec: RustOpaqueCodecMode,
        parse_mode: ParseMode,
        stop_on_error: bool,
    ) -> anyhow::Result<MirFuncOrSkip> {
        match self.parse_function_inner(
            func,
            force_codec_mode_pack,
            default_stream_sink_codec,
            default_rust_opaque_codec,
            parse_mode,
        ) {
            Ok(output) => Ok(output),
            Err(err) => {
                // This will stop the whole generator and tell the users, so we do not care about testing it
                // frb-coverage:ignore-start
                if stop_on_error {
                    Err(err.context(format!(
                        "parse_function halt since stop_on_error=true and see error (function={})",
                        serde_json::to_string(func).unwrap(),
                    )))
                } else {
                    debug!(
                        "parse_function see error and skip function: function={:?} error={:?}",
                        func.item_fn.name(),
                        err
                    );
                    Ok(create_output_skip(func, MirSkipReason::Err))
                }
                // frb-coverage:ignore-end
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn parse_function_inner(
        &mut self,
        func: &HirFlatFunction,
        force_codec_mode_pack: &Option<CodecModePack>,
        default_stream_sink_codec: CodecMode,
        default_rust_opaque_codec: RustOpaqueCodecMode,
        parse_mode: ParseMode,
    ) -> anyhow::Result<MirFuncOrSkip> {
        debug!("parse_function function name: {:?}", func.item_fn.name());

        if func.is_public() == Some(false) {
            return Ok(create_output_skip(func, IgnoredFunctionNotPub));
        }
        if !func.item_fn.sig().generics.params.is_empty() {
            return Ok(create_output_skip(func, IgnoredFunctionGeneric));
        }

        let src_lineno = func.item_fn.span().start().line;
        let attributes = FrbAttributes::parse(func.item_fn.attrs())?;

        let dart_name = parse_dart_name(&attributes, &func.item_fn.name());

        let create_context = |owner: Option<MirFuncOwnerInfo>| TypeParserParsingContext {
            initiated_namespace: func.namespace.clone(),
            func_attributes: attributes.clone(),
            default_stream_sink_codec,
            default_rust_opaque_codec,
            owner,
            parse_mode,
        };

        let is_owner_trait_def = matches!(func.owner, HirFlatFunctionOwner::TraitDef { .. });
        let owner = if let Some(owner) =
            self.parse_owner(func, &create_context(None), dart_name.clone(), &attributes)?
        {
            owner
        } else {
            return Ok(create_output_skip(func, IgnoredMisc));
        };

        let func_name = parse_name(&func.item_fn.name(), &owner);

        if attributes.ignore() {
            return Ok(create_output_skip(func, IgnoredMisc));
        }

        let context = create_context(Some(owner.clone()));
        let mut info = FunctionPartialInfo::default();
        for sig_input in func.item_fn.sig().inputs.iter() {
            info =
                info.merge(self.parse_fn_arg(sig_input, &owner, &context, is_owner_trait_def)?)?;
        }
        info =
            info.merge(self.parse_fn_output(func.item_fn.sig(), &owner, &context, &attributes)?)?;
        info = self.transform_fn_info(info);

        let codec_mode_pack = compute_codec_mode_pack(&attributes, force_codec_mode_pack);
        let mode = compute_func_mode(&attributes, &info);
        let stream_dart_await = attributes.stream_dart_await() && !attributes.sync();
        let namespace_refined = refine_namespace(&owner).unwrap_or(func.namespace.clone());
        let accessor = attributes.accessor();

        let output = MirFuncOutput {
            normal: info.ok_output.unwrap_or(Primitive(MirTypePrimitive::Unit)),
            error: info.error_output,
        };

        let impl_mode = compute_impl_mode(is_owner_trait_def, &func_name, &attributes, &output);

        if info.ignore_func {
            return Ok(create_output_skip(func, IgnoredMisc));
        }

        Ok(MirFuncOrSkip::Func(MirFunc {
            name: NamespacedName::new(namespace_refined, func_name),
            dart_name,
            id: None, // to be filled later
            inputs: info.inputs,
            output,
            owner,
            mode,
            stream_dart_await,
            rust_async: func.item_fn.sig().asyncness.is_some(),
            initializer: attributes.init(),
            accessor,
            arg_mode: if attributes.positional() {
                MirFuncArgMode::Positional
            } else {
                MirFuncArgMode::Named
            },
            comments: parse_comments(func.item_fn.attrs()),
            codec_mode_pack,
            rust_call_code: None,
            impl_mode,
            src_lineno_pseudo: src_lineno,
        }))
    }

    fn parse_owner(
        &mut self,
        func: &HirFlatFunction,
        context: &TypeParserParsingContext,
        actual_method_dart_name: Option<String>,
        attributes: &FrbAttributes,
    ) -> anyhow::Result<Option<MirFuncOwnerInfo>> {
        match &func.owner {
            HirFlatFunctionOwner::Function => Ok(Some(MirFuncOwnerInfo::Function)),
            HirFlatFunctionOwner::StructOrEnum {
                impl_ty,
                trait_def_name,
            } => {
                let owner_ty = if let Some(x) = self.parse_method_owner_ty(impl_ty, context)? {
                    x
                } else {
                    return Ok(None);
                };

                let trait_def = if let Some(trait_def_name) = trait_def_name {
                    if let Some(ans) = parse_type_trait(trait_def_name, self.type_parser) {
                        Some(ans)
                    } else {
                        // If cannot find the trait, we directly skip the function currently
                        return Ok(None);
                    }
                } else {
                    None
                };

                if !is_allowed_owner(&owner_ty, attributes) {
                    return Ok(None);
                }

                self.parse_method_owner_inner(func, actual_method_dart_name, owner_ty, trait_def)
            }
            HirFlatFunctionOwner::TraitDef { trait_def_name } => {
                let trait_def = MirTypeTraitDef {
                    name: trait_def_name.to_owned(),
                };

                self.parse_method_owner_inner(
                    func,
                    actual_method_dart_name,
                    MirType::TraitDef(trait_def.clone()),
                    Some(trait_def),
                )
            }
        }
    }

    fn parse_method_owner_inner(
        &mut self,
        func: &HirFlatFunction,
        actual_method_dart_name: Option<String>,
        owner_ty: MirType,
        trait_def: Option<MirTypeTraitDef>,
    ) -> anyhow::Result<Option<MirFuncOwnerInfo>> {
        let sig = func.item_fn.sig();
        let mode = if matches!(sig.inputs.first(), Some(FnArg::Receiver(..))) {
            MirFuncOwnerInfoMethodMode::Instance
        } else {
            MirFuncOwnerInfoMethodMode::Static
        };

        if owner_ty.should_ignore(self.type_parser) {
            return Ok(None);
        }

        let actual_method_name = sig.ident.to_string();

        Ok(Some(MirFuncOwnerInfo::Method(MirFuncOwnerInfoMethod {
            owner_ty,
            actual_method_name,
            actual_method_dart_name,
            mode,
            trait_def,
        })))
    }

    fn parse_method_owner_ty(
        &mut self,
        impl_ty: &Type,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<Option<MirType>> {
        let self_ty_path = if let Type::Path(self_ty_path) = impl_ty {
            self_ty_path
        } else {
            return Ok(None);
        };

        // let owner_ty_name = external_impl::parse_name_or_original(
        //     &(self_ty_path.path.segments.first().unwrap().ident).to_string(),
        // )?;
        let owner_ty_name = (self_ty_path.path.segments.first().unwrap().ident).to_string();
        let syn_ty: Type = parse_str(&owner_ty_name)?;
        Ok(Some(self.type_parser.parse_type(&syn_ty, context)?))
    }
}

fn create_output_skip(func: &HirFlatFunction, reason: MirSkipReason) -> MirFuncOrSkip {
    MirFuncOrSkip::Skip(MirSkip {
        name: NamespacedName::new(func.namespace.clone(), func.item_fn.name().to_string()),
        reason,
    })
}

fn compute_func_mode(attributes: &FrbAttributes, info: &FunctionPartialInfo) -> MirFuncMode {
    info.mode.unwrap_or(if attributes.sync() {
        MirFuncMode::Sync
    } else {
        MirFuncMode::Normal
    })
}

fn parse_name(function_sig_ident_raw_name: &str, owner: &MirFuncOwnerInfo) -> String {
    match owner {
        MirFuncOwnerInfo::Function => function_sig_ident_raw_name.to_string(),
        MirFuncOwnerInfo::Method(method) => parse_effective_function_name_of_method(method),
    }
}

pub(crate) fn parse_effective_function_name_of_method(method: &MirFuncOwnerInfoMethod) -> String {
    let owner_name = match &method.owner_ty {
        MirType::RustAutoOpaqueImplicit(ty) => ty.sanitized_type(),
        ty => ty.safe_ident(),
    };
    format!("{owner_name}_{}", method.actual_method_name)
}

#[derive(Debug, Default)]
struct FunctionPartialInfo {
    inputs: Vec<MirFuncInput>,
    ok_output: Option<MirType>,
    error_output: Option<MirType>,
    mode: Option<MirFuncMode>,
    ignore_func: bool,
}

impl FunctionPartialInfo {
    fn merge(self, other: Self) -> anyhow::Result<Self> {
        Ok(Self {
            inputs: concat([self.inputs, other.inputs]),
            ok_output: merge_option(self.ok_output, other.ok_output).context("ok_output type")?,
            error_output: merge_option(self.error_output, other.error_output)
                .context("error_output type")?,
            mode: merge_option(self.mode, other.mode).context("mode")?,
            ignore_func: self.ignore_func || other.ignore_func,
        })
    }
}

fn merge_option<T: Debug>(a: Option<T>, b: Option<T>) -> anyhow::Result<Option<T>> {
    if a.is_some() && b.is_some() {
        // This will stop the whole generator and tell the users, so we do not care about testing it
        // frb-coverage:ignore-start
        bail!("Function has conflicting arguments and/or outputs: {a:?} and {b:?}");
        // frb-coverage:ignore-end
    }
    Ok(a.or(b))
}

pub(crate) fn compute_codec_mode_pack(
    attributes: &FrbAttributes,
    force_ans: &Option<CodecModePack>,
) -> CodecModePack {
    const DEFAULT_ANS: CodecModePack = CodecModePack {
        dart2rust: CodecMode::Cst,
        rust2dart: CodecMode::Dco,
    };
    let attr_ans = attributes.codec_mode_pack();

    if force_ans.is_some() && attr_ans.is_some() {
        warn!("Ignore attributes setting codec mode (e.g. when full_dep=false)");
    }

    force_ans.to_owned().or(attr_ans).unwrap_or(DEFAULT_ANS)
}

fn refine_namespace(owner: &MirFuncOwnerInfo) -> Option<Namespace> {
    if let MirFuncOwnerInfo::Method(method) = owner {
        method.owner_ty.self_namespace()
    } else {
        None
    }
}

fn is_allowed_owner(owner_ty: &MirType, attributes: &FrbAttributes) -> bool {
    // if `#[frb(external)]`, then allow arbitrary type
    attributes.external() || is_struct_or_enum_or_opaque_from_them(owner_ty)
}

pub(crate) fn is_struct_or_enum_or_opaque_from_them(ty: &MirType) -> bool {
    match ty {
        MirType::StructRef(_)
        | MirType::EnumRef(_)
        | MirType::Delegate(MirTypeDelegate::PrimitiveEnum(_)) => true,
        MirType::RustAutoOpaqueImplicit(ty) => {
            ty.reason == Some(MirTypeRustAutoOpaqueImplicitReason::StructOrEnumRequireOpaque)
        }
        _ => false,
    }
}

pub(crate) const FUNC_PREFIX_FRB_INTERNAL_NO_IMPL: &str = "frb_internal_no_impl";

fn compute_impl_mode(
    is_owner_trait_def: bool,
    func_name: &str,
    attributes: &FrbAttributes,
    output: &MirFuncOutput,
) -> MirFuncImplMode {
    if is_owner_trait_def || func_name.starts_with(FUNC_PREFIX_FRB_INTERNAL_NO_IMPL) {
        return MirFuncImplMode::NoImpl;
    }

    if attributes.proxy() {
        if let MirType::Delegate(MirTypeDelegate::ProxyVariant(inner)) = &output.normal {
            return MirFuncImplMode::DartOnly(MirFuncImplModeDartOnly::CreateProxyVariant(
                inner.clone(),
            ));
        }
    }

    MirFuncImplMode::Normal
}

fn parse_dart_name(attributes: &FrbAttributes, func_name_raw: &str) -> Option<String> {
    (attributes.name()).or_else(|| {
        (attributes.accessor()).and_then(|accessor| {
            (func_name_raw.strip_prefix(&format!("{}_", accessor.verb_str())))
                .map(ToString::to_string)
        })
    })
}
