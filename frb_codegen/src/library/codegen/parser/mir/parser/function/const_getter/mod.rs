use crate::codegen::ir::hir::flat::constant::HirFlatConstant;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncAccessorMode, MirFuncArgMode, MirFuncImplMode, MirFuncMode, MirFuncOutput,
    MirFuncOwnerInfo,
};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::misc::skip::IrSkipReason::{
    IgnoreBecauseExplicitAttribute, IgnoreBecauseFunctionNotPub,
};
use crate::codegen::ir::misc::skip::{IrSkip, IrSkipReason, IrValueOrSkip, MirFuncOrSkip};
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::auto_accessor::create_simplified_parsing_context;
use crate::codegen::parser::mir::parser::function::real::compute_codec_mode_pack;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::ParseMode;
use crate::utils::namespace::NamespacedName;
use syn::spanned::Spanned;
use syn::Visibility;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    src_constants: &[HirFlatConstant],
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    Ok((src_constants.iter())
        .map(|constant| parse_constant(config, type_parser, parse_mode, constant))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect())
}

fn parse_constant(
    config: &ParserMirInternalConfig,
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
    constant: &HirFlatConstant,
) -> anyhow::Result<Option<IrValueOrSkip<MirFunc, IrSkip>>> {
    let namespace = &constant.namespace;
    let name = constant.item_const.ident.to_string();
    let context = create_simplified_parsing_context(namespace.clone(), config, parse_mode)?;
    let attributes = FrbAttributes::parse(&constant.item_const.attrs)?;

    // reserved name
    if &name == "_" {
        return Ok(None);
    }
    if !matches!(&constant.item_const.vis, Visibility::Public(_)) {
        return Ok(create_output_skip(constant, IgnoreBecauseFunctionNotPub));
    }
    if attributes.ignore() {
        return Ok(create_output_skip(constant, IgnoreBecauseExplicitAttribute));
    }

    let ty_direct_parse = match type_parser.parse_type(&constant.item_const.ty, &context) {
        Ok(value) => value,
        // We do not care about parsing errors here (e.g. some type that we do not support)
        Err(_) => return Ok(create_output_skip(constant, IrSkipReason::Err)),
    };

    let rust_call_code = format!("{}::{name}", namespace.joined_path);

    Ok(Some(MirFuncOrSkip::Value(MirFunc {
        namespace: namespace.clone(),
        name: MirIdent::new(name, None),
        id: None,
        inputs: vec![],
        output: MirFuncOutput {
            normal: ty_direct_parse,
            error: None,
        },
        owner: MirFuncOwnerInfo::Function,
        mode: MirFuncMode::Sync,
        stream_dart_await: false,
        rust_async: false,
        initializer: false,
        arg_mode: MirFuncArgMode::Positional,
        accessor: Some(MirFuncAccessorMode::Getter),
        comments: vec![],
        codec_mode_pack: compute_codec_mode_pack(
            &FrbAttributes::parse(&[])?,
            &config.force_codec_mode_pack,
        ),
        rust_call_code: Some(rust_call_code),
        rust_aop_after: None,
        impl_mode: MirFuncImplMode::Normal,
        src_lineno_pseudo: constant.item_const.span().start().line,
    })))
}

fn create_output_skip(constant: &HirFlatConstant, reason: IrSkipReason) -> Option<MirFuncOrSkip> {
    Some(IrValueOrSkip::Skip(IrSkip {
        name: NamespacedName::new(
            constant.namespace.clone(),
            constant.item_const.ident.to_string(),
        ),
        reason,
    }))
}
