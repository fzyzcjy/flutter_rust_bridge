use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::hir::flat::function::HirFlatFunctionOwner;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncArgMode, MirFuncInput, MirFuncMode, MirFuncOutput, MirFuncOverridePriority,
    MirFuncOwnerInfo, MirFuncOwnerInfoMethod, MirFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::mir::skip::MirSkipReason::IgnoredFunctionGeneric;
use crate::codegen::ir::mir::skip::{MirSkip, MirSkipReason};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::attribute_parser::FrbAttributes;
use crate::codegen::parser::mir::function_parser::structs::ParseFunctionOutput;
use crate::codegen::parser::mir::type_parser::misc::parse_comments;
use crate::codegen::parser::mir::type_parser::{TypeParser, TypeParserParsingContext};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::{Namespace, NamespacedName};
use anyhow::{bail, Context};
use itertools::concat;
use log::{debug, warn};
use std::fmt::Debug;
use syn::*;
use MirSkipReason::{IgnoredFunctionNotPub, IgnoredMisc};
use MirType::Primitive;

pub(crate) mod argument;
pub(crate) mod output;
pub(crate) mod structs;
mod transformer;

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
        default_stream_sink_codec: CodecMode,
        default_rust_opaque_codec: RustOpaqueCodecMode,
    ) -> anyhow::Result<ParseFunctionOutput> {
        match self.parse_function_inner(
            func,
            force_codec_mode_pack,
            default_stream_sink_codec,
            default_rust_opaque_codec,
        ) {
            Ok(output) => Ok(output),
            Err(err) => {
                log::debug!(
                    "parse_function see error and skip function: function={:?} error={:?}",
                    func.item_fn.name(),
                    err
                );
                Ok(create_output_skip(func, MirSkipReason::Err))
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
    ) -> anyhow::Result<ParseFunctionOutput> {
        debug!("parse_function function name: {:?}", func.item_fn.name());

        if func.is_public() == Some(false) {
            return Ok(create_output_skip(func, IgnoredFunctionNotPub));
        }
        if !func.item_fn.sig().generics.params.is_empty() {
            return Ok(create_output_skip(func, IgnoredFunctionGeneric));
        }

        let src_lineno = func.item_fn.span().start().line;
        let attributes = FrbAttributes::parse(func.item_fn.attrs())?;

        let dart_name = attributes.name();
        let override_priority = MirFuncOverridePriority::default();
        let (dart_name, override_priority) =
            parse_frb_override_marker(dart_name, override_priority, func);

        let create_context = |owner: Option<MirFuncOwnerInfo>| TypeParserParsingContext {
            initiated_namespace: func.namespace.clone(),
            func_attributes: attributes.clone(),
            default_stream_sink_codec,
            default_rust_opaque_codec,
            owner,
        };

        let owner = if let Some(owner) =
            self.parse_owner(func, &create_context(None), dart_name.clone())?
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
            info = info.merge(self.parse_fn_arg(sig_input, &owner, &context)?)?;
        }
        info = info.merge(self.parse_fn_output(func.item_fn.sig(), &context)?)?;
        info = self.transform_fn_info(info);

        let codec_mode_pack = compute_codec_mode_pack(&attributes, force_codec_mode_pack);
        let mode = compute_func_mode(&attributes, &info);
        let stream_dart_await = attributes.stream_dart_await() && !attributes.sync();
        let namespace_refined = refine_namespace(&owner).unwrap_or(func.namespace.clone());

        if info.ignore_func {
            return Ok(create_output_skip(func, IgnoredMisc));
        }

        Ok(ParseFunctionOutput::Ok(MirFunc {
            name: NamespacedName::new(namespace_refined, func_name),
            dart_name,
            id: None, // to be filled later
            inputs: info.inputs,
            output: MirFuncOutput {
                normal: info.ok_output.unwrap_or(Primitive(MirTypePrimitive::Unit)),
                error: info.error_output,
            },
            owner,
            mode,
            stream_dart_await,
            rust_async: func.item_fn.sig().asyncness.is_some(),
            initializer: attributes.init(),
            accessor: attributes.accessor(),
            arg_mode: if attributes.positional() {
                MirFuncArgMode::Positional
            } else {
                MirFuncArgMode::Named
            },
            comments: parse_comments(func.item_fn.attrs()),
            codec_mode_pack,
            rust_call_code: None,
            src_lineno_pseudo: src_lineno,
            override_priority,
        }))
    }

    fn parse_owner(
        &mut self,
        func: &HirFlatFunction,
        context: &TypeParserParsingContext,
        actual_method_dart_name: Option<String>,
    ) -> anyhow::Result<Option<MirFuncOwnerInfo>> {
        Ok(Some(match &func.owner {
            HirFlatFunctionOwner::TraitDef { .. } => return Ok(None), // TODO not yet implemented
            HirFlatFunctionOwner::Function => MirFuncOwnerInfo::Function,
            HirFlatFunctionOwner::StructOrEnum {
                impl_ty,
                trait_def_name,
            } => {
                let sig = func.item_fn.sig();
                let mode = if matches!(sig.inputs.first(), Some(FnArg::Receiver(..))) {
                    MirFuncOwnerInfoMethodMode::Instance
                } else {
                    MirFuncOwnerInfoMethodMode::Static
                };

                let owner_ty = if let Some(x) = self.parse_method_owner_ty(impl_ty, context)? {
                    x
                } else {
                    return Ok(None);
                };

                if owner_ty.should_ignore(self.type_parser) {
                    return Ok(None);
                }

                let actual_method_name = sig.ident.to_string();

                let trait_def_namespaced_name = if let Some(trait_def_name) = trait_def_name {
                    if let Some(ans) = self.type_parser.src_traits.get(trait_def_name) {
                        Some(ans.name.clone())
                    } else {
                        return Ok(None);
                    }
                } else {
                    None
                };

                MirFuncOwnerInfo::Method(MirFuncOwnerInfoMethod {
                    owner_ty,
                    actual_method_name,
                    actual_method_dart_name,
                    mode,
                    trait_def_name: trait_def_namespaced_name,
                })
            }
        }))
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

fn create_output_skip(func: &HirFlatFunction, reason: MirSkipReason) -> ParseFunctionOutput {
    ParseFunctionOutput::Skip(MirSkip {
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
        let owner_ty = &method.owner_ty;
        if matches!(owner_ty, MirType::StructRef(_) | MirType::EnumRef(_)) {
            return owner_ty.self_namespace();
        }
    }
    None
}

fn parse_frb_override_marker(
    dart_name_raw: Option<String>,
    override_priority_raw: MirFuncOverridePriority,
    func: &HirFlatFunction,
) -> (Option<String>, MirFuncOverridePriority) {
    const FRB_OVERRIDE_PREFIX: &str = "frb_override_";
    if let Some(func_name_stripped) = func.item_fn.name().strip_prefix(FRB_OVERRIDE_PREFIX) {
        (
            dart_name_raw.or(Some(func_name_stripped.to_owned())),
            MirFuncOverridePriority::FRB_OVERRIDE,
        )
    } else {
        (dart_name_raw, override_priority_raw)
    }
}
