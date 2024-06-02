use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
use crate::codegen::ir::mir::func::{MirFuncInput, MirFuncOwnerInfo};
use crate::codegen::ir::mir::func::{MirFuncOwnerInfoMethod, OwnershipMode};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::Boxed;
use crate::codegen::parser::mir::attribute_parser::FrbAttributes;
use crate::codegen::parser::mir::function_parser::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::mir::type_parser::misc::parse_comments;
use crate::codegen::parser::mir::type_parser::rust_auto_opaque_implicit::split_ownership_from_ty;
use crate::codegen::parser::mir::type_parser::{TypeParser, TypeParserParsingContext};
use crate::if_then_some;
use anyhow::Context;
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_arg(
        &mut self,
        sig_input: &FnArg,
        owner: &MirFuncOwnerInfo,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let (ty_syn_raw, name) = match sig_input {
            FnArg::Typed(ref pat_type) => {
                (*pat_type.ty.clone(), parse_name_from_pat_type(pat_type)?)
            }
            FnArg::Receiver(ref receiver) => {
                let method = if_then_some!(let MirFuncOwnerInfo::Method(method) = owner, method)
                    .context("`self` must happen within methods")?;
                (
                    syntheize_receiver_type(receiver, method)?,
                    "that".to_owned(),
                )
            }
        };

        let (ty_syn_without_ownership, ownership_mode_split) =
            split_ownership_from_ty_except_ref_mut(&ty_syn_raw);

        let ty_without_ownership =
            (self.type_parser).parse_type(&ty_syn_without_ownership, context)?;

        let (ty, ownership_mode) = merge_ownership_into_ty(
            self.type_parser,
            context,
            ty_without_ownership,
            ownership_mode_split,
        )?;

        if ty.should_ignore(self.type_parser) {
            return Ok(FunctionPartialInfo {
                ignore_func: true,
                ..Default::default()
            });
        }

        let attrs = parse_attrs_from_fn_arg(sig_input);
        let attributes = FrbAttributes::parse(attrs)?;
        let ty = auto_add_boxed(ty);
        Ok(FunctionPartialInfo {
            inputs: vec![MirFuncInput {
                inner: MirField {
                    name: MirIdent::new(name),
                    ty,
                    is_final: true,
                    is_rust_public: None,
                    comments: parse_comments(attrs),
                    default: attributes.default_value(),
                    settings: MirFieldSettings::default(),
                },
                ownership_mode,
            }],
            ..Default::default()
        })
    }
}

pub(crate) fn merge_ownership_into_ty(
    type_parser: &mut TypeParser,
    context: &TypeParserParsingContext,
    ty_without_ownership: MirType,
    ownership_mode: Option<OwnershipMode>,
) -> anyhow::Result<(MirType, Option<OwnershipMode>)> {
    Ok(match (ty_without_ownership, ownership_mode) {
        (MirType::RustAutoOpaqueImplicit(ty_raw), Some(ownership_mode)) => (
            type_parser.transform_rust_auto_opaque(
                &ty_raw,
                |raw| format!("{}{raw}", ownership_mode.prefix()),
                context,
            )?,
            None,
        ),
        others => others,
    })
}

fn auto_add_boxed(ty: MirType) -> MirType {
    if ty.is_struct_or_enum_or_record() {
        Boxed(MirTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty),
        })
    } else {
        ty
    }
}

fn parse_name_from_pat_type(pat_type: &PatType) -> anyhow::Result<String> {
    if_then_some!(let Pat::Ident(ref pat_ident) = *pat_type.pat, pat_ident)
        .map(|pat_ident| format!("{}", pat_ident.ident))
        .with_context(|| quote::quote!(#pat_type).to_string())
}

fn syntheize_receiver_type(
    receiver: &Receiver,
    method: &MirFuncOwnerInfoMethod,
) -> anyhow::Result<Type> {
    let ty_str = format!(
        "{}{}",
        parse_receiver_ownership_mode(receiver).prefix(),
        method.owner_ty_name().name.to_owned()
    );
    Ok(parse_str::<Type>(&ty_str)?)
}

fn parse_receiver_ownership_mode(receiver: &Receiver) -> OwnershipMode {
    if receiver.reference.is_some() {
        if receiver.mutability.is_some() {
            OwnershipMode::RefMut
        } else {
            OwnershipMode::Ref
        }
    } else {
        OwnershipMode::Owned
    }
}

pub(crate) fn split_ownership_from_ty_except_ref_mut(
    ty_raw: &Type,
) -> (Type, Option<OwnershipMode>) {
    let (ty, ownership_mode) = split_ownership_from_ty(ty_raw);
    if ownership_mode == OwnershipMode::RefMut {
        (ty_raw.to_owned(), None)
    } else {
        (ty, Some(ownership_mode))
    }
}

fn parse_attrs_from_fn_arg(fn_arg: &FnArg) -> &[Attribute] {
    match fn_arg {
        FnArg::Typed(inner) => &inner.attrs,
        FnArg::Receiver(inner) => &inner.attrs,
    }
}
