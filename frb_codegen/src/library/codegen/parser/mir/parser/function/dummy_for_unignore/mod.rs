use crate::codegen::ir::hir::flat::constant::HirFlatConstant;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncAccessorMode, MirFuncArgMode, MirFuncImplMode, MirFuncMode, MirFuncOutput,
    MirFuncOwnerInfo,
};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::misc::skip::{IrSkip, IrSkipReason, IrValueOrSkip, MirFuncOrSkip};
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::auto_accessor::create_simplified_parsing_context;
use crate::codegen::parser::mir::parser::function::real::compute_codec_mode_pack;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use crate::codegen::parser::mir::ParseMode;
use crate::utils::namespace::NamespacedName;
use syn::spanned::Spanned;

pub(crate) fn parse(
    config: &ParserMirInternalConfig,
    type_parser: &mut TypeParser,
    parse_mode: ParseMode,
) -> anyhow::Result<Vec<MirFuncOrSkip>> {
    TODO
}
