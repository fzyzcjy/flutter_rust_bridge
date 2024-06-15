use crate::codegen::ir::hir::flat::function::{HirFlatFunction, HirFlatFunctionOwner};
use crate::codegen::ir::mir::func::{
    MirFuncOwnerInfo, MirFuncOwnerInfoMethod, MirFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::mir::skip::MirSkipReason;
use crate::codegen::ir::mir::skip::MirSkipReason::{
    IgnoreBecauseNotAllowedOwner, IgnoreBecauseOwnerTyShouldIgnore,
    IgnoreBecauseParseMethodOwnerTy, IgnoreBecauseParseOwnerCannotFindTrait,
};
use crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::real::{
    is_struct_or_enum_or_opaque_from_them, FunctionParser,
};
use crate::codegen::parser::mir::parser::ty::trait_def::parse_type_trait;
use crate::codegen::parser::mir::parser::ty::TypeParserParsingContext;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use syn::{parse_str, FnArg, Type};

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_owner(
        &mut self,
        func: &HirFlatFunction,
        context: &TypeParserParsingContext,
        actual_method_dart_name: Option<String>,
        attributes: &FrbAttributes,
    ) -> anyhow::Result<OwnerInfoOrSkip> {
        use crate::library::codegen::ir::mir::skip::MirSkipReason::*;
        use OwnerInfoOrSkip::*;

        match &func.owner {
            HirFlatFunctionOwner::Function => Ok(Info(MirFuncOwnerInfo::Function)),
            HirFlatFunctionOwner::StructOrEnum {
                impl_ty,
                trait_def_name,
            } => {
                let owner_ty = if let Some(x) = self.parse_method_owner_ty(impl_ty, context)? {
                    x
                } else {
                    return Ok(Skip(IgnoreBecauseParseMethodOwnerTy));
                };

                let trait_def = if let Some(trait_def_name) = trait_def_name {
                    if let Some(ans) = parse_type_trait(trait_def_name, self.type_parser) {
                        Some(ans)
                    } else {
                        // If cannot find the trait, we directly skip the function currently
                        return Ok(Skip(IgnoreBecauseParseOwnerCannotFindTrait));
                    }
                } else {
                    None
                };

                if !is_allowed_owner(&owner_ty, attributes) {
                    return Ok(Skip(IgnoreBecauseNotAllowedOwner));
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
    ) -> anyhow::Result<OwnerInfoOrSkip> {
        use OwnerInfoOrSkip::*;

        let sig = func.item_fn.sig();
        let mode = if matches!(sig.inputs.first(), Some(FnArg::Receiver(..))) {
            MirFuncOwnerInfoMethodMode::Instance
        } else {
            MirFuncOwnerInfoMethodMode::Static
        };

        if owner_ty.should_ignore(self.type_parser) {
            return Ok(Skip(IgnoreBecauseOwnerTyShouldIgnore));
        }

        let actual_method_name = sig.ident.to_string();

        Ok(Info(MirFuncOwnerInfo::Method(MirFuncOwnerInfoMethod {
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
        // let self_ty_path = if let Type::Path(self_ty_path) = impl_ty {
        //     self_ty_path
        // } else {
        //     return Ok(None);
        // };
        //
        // // let owner_ty_name = external_impl::parse_name_or_original(
        // //     &(self_ty_path.path.segments.first().unwrap().ident).to_string(),
        // // )?;
        // let owner_ty_name = (self_ty_path.path.segments.first().unwrap().ident).to_string();
        // let syn_ty: Type = parse_str(&owner_ty_name)?;
        // Ok(Some(self.type_parser.parse_type(&syn_ty, context)?))
        Ok(Some(self.type_parser.parse_type(impl_ty, context)?))
    }
}

pub(super) enum OwnerInfoOrSkip {
    Info(MirFuncOwnerInfo),
    Skip(MirSkipReason),
}

fn is_allowed_owner(owner_ty: &MirType, attributes: &FrbAttributes) -> bool {
    // if `#[frb(external)]`, then allow arbitrary type
    attributes.external() || is_struct_or_enum_or_opaque_from_them(owner_ty)
}
