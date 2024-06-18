use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::function::HirFlatFunctionOwner;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncArgMode, MirFuncImplMode, MirFuncImplModeDartOnly, MirFuncInput, MirFuncMode,
    MirFuncOutput, MirFuncOwnerInfo, MirFuncOwnerInfoMethod,
};
use crate::codegen::ir::mir::skip::MirSkipReason::{
    IgnoreBecauseExplicitAttribute, IgnoreBecauseFunctionGeneric, IgnoreSilently,
};
use crate::codegen::ir::mir::skip::{MirSkip, MirSkipReason};
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicitReason;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::func_or_skip::MirFuncOrSkip;
use crate::codegen::parser::mir::parser::function::real::lifetime::parse_function_lifetime;
use crate::codegen::parser::mir::parser::function::real::owner::OwnerInfoOrSkip;
use crate::codegen::parser::mir::parser::ty::generics::should_ignore_because_generics;
use crate::codegen::parser::mir::parser::ty::misc::parse_comments;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use crate::codegen::parser::mir::ParseMode;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::{Namespace, NamespacedName};
use anyhow::{bail, Context};
use itertools::concat;
use log::{debug, warn};
use std::fmt::Debug;
use MirSkipReason::IgnoreBecauseFunctionNotPub;
use MirType::Primitive;

pub(crate) mod argument;
pub(super) mod lifetime;
pub(crate) mod output;
mod owner;
mod transformer;

pub(crate) fn parse(
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
    config: &ParserMirInternalConfig,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    let mut function_parser = FunctionParser::new(type_parser);
    (src_fns.iter())
        .map(|f| {
            function_parser.parse_function(
                f,
                &config.force_codec_mode_pack,
                config
                    .rust_input_namespace_pack
                    .rust_output_path_namespace
                    .clone(),
                config.default_stream_sink_codec,
                config.default_rust_opaque_codec,
                config.enable_lifetime,
                config.type_64bit_int,
                parse_mode,
                config.stop_on_error,
            )
        })
        .collect()
}

pub(crate) struct FunctionParser<'a, 'b> {
    type_parser: &'a mut TypeParser<'b>,
}

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(crate) fn new(type_parser: &'a mut TypeParser<'b>) -> Self {
        Self { type_parser }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn parse_function(
        &mut self,
        func: &HirFlatFunction,
        force_codec_mode_pack: &Option<CodecModePack>,
        rust_output_path_namespace: Namespace,
        default_stream_sink_codec: CodecMode,
        default_rust_opaque_codec: RustOpaqueCodecMode,
        enable_lifetime: bool,
        type_64bit_int: bool,
        parse_mode: ParseMode,
        stop_on_error: bool,
    ) -> anyhow::Result<MirFuncOrSkip> {
        match self.parse_function_inner(
            func,
            force_codec_mode_pack,
            rust_output_path_namespace,
            default_stream_sink_codec,
            default_rust_opaque_codec,
            enable_lifetime,
            type_64bit_int,
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
        rust_output_path_namespace: Namespace,
        default_stream_sink_codec: CodecMode,
        default_rust_opaque_codec: RustOpaqueCodecMode,
        enable_lifetime: bool,
        type_64bit_int: bool,
        parse_mode: ParseMode,
    ) -> anyhow::Result<MirFuncOrSkip> {
        debug!("parse_function function name: {:?}", func.item_fn.name());

        if func.is_public() == Some(false) {
            return Ok(create_output_skip(func, IgnoreBecauseFunctionNotPub));
        }

        // If enable lifetime, the lifetime "generics" should be acceptable (though other generics still not)
        if should_ignore_because_generics(&func.item_fn.sig().generics, enable_lifetime) {
            return Ok(create_output_skip(func, IgnoreBecauseFunctionGeneric));
        }

        let src_lineno = func.item_fn.span().start().line;
        let attributes = FrbAttributes::parse(func.item_fn.attrs())?;
        if attributes.dart2rust().is_some() || attributes.rust2dart().is_some() {
            return Ok(create_output_skip(func, IgnoreSilently));
        }

        let dart_name = parse_dart_name(&attributes, &func.item_fn.name());

        let create_context =
            |owner: Option<MirFuncOwnerInfo>, forbid_type_self: bool| TypeParserParsingContext {
                initiated_namespace: func.namespace.clone(),
                func_attributes: attributes.clone(),
                struct_or_enum_attributes: None,
                rust_output_path_namespace: rust_output_path_namespace.clone(),
                default_stream_sink_codec,
                default_rust_opaque_codec,
                owner,
                enable_lifetime,
                type_64bit_int,
                forbid_type_self,
                parse_mode,
            };

        let is_owner_trait_def = matches!(func.owner, HirFlatFunctionOwner::TraitDef { .. });
        let owner = match self.parse_owner(
            func,
            &create_context(None, false),
            dart_name.clone(),
            &attributes,
        )? {
            OwnerInfoOrSkip::Info(info) => info,
            OwnerInfoOrSkip::Skip(reason) => return Ok(create_output_skip(func, reason)),
        };

        let func_name = parse_name(&func.item_fn.name(), &owner);

        if attributes.ignore() {
            return Ok(create_output_skip(func, IgnoreBecauseExplicitAttribute));
        }

        let lifetime_info = parse_function_lifetime(func.item_fn.sig(), &owner)?;

        let context_input = create_context(Some(owner.clone()), TODO);
        let context_output = create_context(Some(owner.clone()), false);
        let mut info = FunctionPartialInfo::default();
        for (i, sig_input) in func.item_fn.sig().inputs.iter().enumerate() {
            info = info.merge(self.parse_fn_arg(
                sig_input,
                &owner,
                &context_input,
                is_owner_trait_def,
                lifetime_info.needs_extend_lifetime_per_arg[i],
            )?)?;
        }
        info = info.merge(self.parse_fn_output(
            func.item_fn.sig(),
            &owner,
            &context_output,
            &attributes,
        )?)?;
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

        if let Some(ignore_func) = info.ignore_func {
            return Ok(create_output_skip(func, ignore_func));
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
    ignore_func: Option<MirSkipReason>,
}

impl FunctionPartialInfo {
    fn merge(self, other: Self) -> anyhow::Result<Self> {
        Ok(Self {
            inputs: concat([self.inputs, other.inputs]),
            ok_output: merge_option(self.ok_output, other.ok_output).context("ok_output type")?,
            error_output: merge_option(self.error_output, other.error_output)
                .context("error_output type")?,
            mode: merge_option(self.mode, other.mode).context("mode")?,
            ignore_func: merge_option(self.ignore_func, other.ignore_func)
                .context("ignore_func")?,
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

pub(crate) fn is_struct_or_enum_or_opaque_from_them(ty: &MirType) -> bool {
    match ty {
        MirType::StructRef(_)
        | MirType::EnumRef(_)
        | MirType::Delegate(MirTypeDelegate::PrimitiveEnum(_)) => true,
        MirType::RustAutoOpaqueImplicit(ty) => {
            ty.reason == Some(MirTypeRustAutoOpaqueImplicitReason::StructOrEnumRequireOpaque)
        }
        MirType::Delegate(MirTypeDelegate::Lifetimeable(ty)) => {
            is_struct_or_enum_or_opaque_from_them(&MirType::RustAutoOpaqueImplicit(
                ty.api_type.clone(),
            ))
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
