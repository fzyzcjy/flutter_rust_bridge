use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::{IrFuncMode, IrFuncOwnerInfo};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::rust_auto_opaque::OwnershipMode;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Boxed;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::function_parser::{
    FunctionParser, FunctionPartialInfo,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParserParsingContext;
use crate::if_then_some;
use anyhow::{bail, ensure, Context};
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_arg(
        &mut self,
        argument_index: usize,
        sig_input: &FnArg,
        owner: &IrFuncOwnerInfo,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<FunctionPartialInfo> {
        match sig_input {
            FnArg::Typed(ref pat_type) => {
                self.parse_fn_arg_typed(argument_index, context, pat_type)
            }
            FnArg::Receiver(ref receiver) => self.parse_fn_arg_receiver(owner, context, receiver),
        }
    }

    fn parse_fn_arg_typed(
        &mut self,
        argument_index: usize,
        context: &TypeParserParsingContext,
        pat_type: &PatType,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let ty_raw = self.type_parser.parse_type(pat_type.ty.as_ref(), context)?;
        let name = parse_name_from_pat_type(pat_type)?;
        partial_info_for_normal_type_raw(ty_raw, &pat_type.attrs, name)
    }

    fn parse_fn_arg_receiver(
        &mut self,
        owner: &IrFuncOwnerInfo,
        context: &TypeParserParsingContext,
        receiver: &Receiver,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let method = if_then_some!(let IrFuncOwnerInfo::Method(method) = owner, method)
            .context("`self` must happen within methods")?;

        let ty_raw = self.type_parser.parse_type(
            &parse_str::<Type>(&method.enum_or_struct_name.name)?,
            context,
        )?;
        let ty = match ty_raw {
            IrType::RustAutoOpaque(ty_raw) => self.type_parser.transform_rust_auto_opaque(
                &ty_raw,
                |raw| generate_ref_type_considering_reference(raw, receiver),
                context,
            )?,
            _ => ty_raw,
        };

        let name = "that".to_owned();

        if let IrType::StructRef(s) = &ty {
            if s.get(self.type_parser).ignore {
                return Ok(FunctionPartialInfo {
                    ignore_func: true,
                    ..Default::default()
                });
            }

            ensure!(
                parse_receiver_ownership_mode(receiver) == OwnershipMode::Ref,
                "If you want to use `self`/`&mut self`, please make the struct opaque (by adding `#[frb(opaque)]` on the struct)."
            );
        }

        partial_info_for_normal_type_raw(ty, &receiver.attrs, name)
    }
}

fn partial_info_for_normal_type_raw(
    ty_raw: IrType,
    attrs: &[Attribute],
    name: String,
) -> anyhow::Result<FunctionPartialInfo> {
    let attributes = FrbAttributes::parse(attrs)?;
    let ty = auto_add_boxed(ty_raw);
    Ok(FunctionPartialInfo {
        inputs: vec![IrField {
            name: IrIdent::new(name),
            ty,
            is_final: true,
            comments: parse_comments(attrs),
            default: attributes.default_value(),
            settings: IrFieldSettings::default(),
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

fn generate_ref_type_considering_reference(raw: &str, receiver: &Receiver) -> String {
    match parse_receiver_ownership_mode(receiver) {
        OwnershipMode::Owned => raw.to_owned(),
        OwnershipMode::RefMut => format!("&mut {raw}"),
        OwnershipMode::Ref => format!("&{raw}"),
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
