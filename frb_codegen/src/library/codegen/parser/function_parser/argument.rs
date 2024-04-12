use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::OwnershipMode;
use crate::codegen::ir::func::{IrFuncInput, IrFuncOwnerInfo};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Boxed;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::function_parser::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParserParsingContext;
use crate::if_then_some;
use anyhow::{bail, ensure, Context};
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_arg(
        &mut self,
        sig_input: &FnArg,
        owner: &IrFuncOwnerInfo,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<FunctionPartialInfo> {
        match sig_input {
            FnArg::Typed(ref pat_type) => self.parse_fn_arg_typed(context, pat_type),
            FnArg::Receiver(ref receiver) => self.parse_fn_arg_receiver(owner, context, receiver),
        }
    }

    fn parse_fn_arg_typed(
        &mut self,
        context: &TypeParserParsingContext,
        pat_type: &PatType,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let (ty, ownership_mode) =
            self.parse_fn_arg_common(&remove_ownership(pat_type.ty.as_ref()), TODO)?;
        let name = parse_name_from_pat_type(pat_type)?;
        partial_info_for_normal_type_raw(ty_raw, &pat_type.attrs, name, ownership_mode)
    }

    fn parse_fn_arg_receiver(
        &mut self,
        owner: &IrFuncOwnerInfo,
        context: &TypeParserParsingContext,
        receiver: &Receiver,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let method = if_then_some!(let IrFuncOwnerInfo::Method(method) = owner, method)
            .context("`self` must happen within methods")?;

        let (ty, ownership_mode) = self.parse_fn_arg_common(
            &parse_str::<Type>(&method.owner_ty_name().name)?,
            parse_receiver_ownership_mode(receiver),
        )?;
        let name = "that".to_owned();

        if let IrType::StructRef(s) = &ty {
            if s.get(self.type_parser).ignore {
                return Ok(FunctionPartialInfo {
                    ignore_func: true,
                    ..Default::default()
                });
            }

            ensure!(
                ownership_mode != Some(OwnershipMode::RefMut),
                "If you want to use `&mut self`, please make the struct opaque (by adding `#[frb(opaque)]` on the struct)."
            );
        }

        partial_info_for_normal_type_raw(ty, &receiver.attrs, name, ownership_mode)
    }

    fn parse_fn_arg_common(
        &mut self,
        ty_syn_without_ownership: &Type,
        ownership_mode_raw: OwnershipMode,
    ) -> anyhow::Result<(IrType, Option<OwnershipMode>)> {
        let ty_raw = self
            .type_parser
            .parse_type(ty_syn_without_ownership, context)?;
        Ok(match ty_raw {
            IrType::RustAutoOpaque(ty_raw) => (
                self.type_parser.transform_rust_auto_opaque(
                    &ty_raw,
                    |raw| match ownership_mode_raw {
                        OwnershipMode::Owned => raw.to_owned(),
                        OwnershipMode::RefMut => format!("&mut {raw}"),
                        OwnershipMode::Ref => format!("&{raw}"),
                    },
                    context,
                )?,
                None,
            ),
            _ => (ty_raw, Some(ownership_mode_raw)),
        })
    }
}

fn partial_info_for_normal_type_raw(
    ty_raw: IrType,
    attrs: &[Attribute],
    name: String,
    ownership_mode: Option<OwnershipMode>,
) -> anyhow::Result<FunctionPartialInfo> {
    let attributes = FrbAttributes::parse(attrs)?;
    let ty = auto_add_boxed(ty_raw);
    Ok(FunctionPartialInfo {
        inputs: vec![IrFuncInput {
            inner: IrField {
                name: IrIdent::new(name),
                ty,
                is_final: true,
                comments: parse_comments(attrs),
                default: attributes.default_value(),
                settings: IrFieldSettings::default(),
            },
            ownership_mode,
        }],
        ..Default::default()
    })
}

fn auto_add_boxed(ty: IrType) -> IrType {
    if ty.is_struct_or_enum_or_record() {
        Boxed(IrTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty),
        })
    } else {
        ty
    }
}

fn parse_name_from_pat_type(pat_type: &PatType) -> anyhow::Result<String> {
    if let Pat::Ident(ref pat_ident) = *pat_type.pat {
        Ok(format!("{}", pat_ident.ident))
    } else {
        // This branch simply halts the generator with an error message, so we do not test it
        // frb-coverage:ignore-start
        bail!(
            "Unexpected pattern: {}",
            quote::quote!(#pat_type).to_string(),
        )
        // frb-coverage:ignore-end
    }
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

fn parse_and_remove_ownership(ty: &Type) -> (Type, OwnershipMode) {
    if let Type::Reference(ty_inner) = ty {
        (
            *ty_inner.elem,
            if ty_inner.mutability.is_some() {
                OwnershipMode::RefMut
            } else {
                OwnershipMode::Ref
            },
        )
    } else {
        (ty.to_owned(), OwnershipMode::Owned)
    }
}
