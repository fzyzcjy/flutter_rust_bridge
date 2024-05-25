use crate::codegen::ir::field::IrField;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;
use std::collections::HashSet;

pub(crate) fn check_opaque_inside_translatable(pack: &IrPack) {
    (pack.distinct_types(None).into_iter())
        .map(|ty| handle_type(pack, ty))
        .collect()
}

fn handle_type(pack: &IrPack, ty: IrType) -> Vec<TODO> {
    match ty {
        IrType::StructRef(ty) => {
            let st = ty.get(pack);
            handle_struct(st, ty.ident.0.rust_style())
        }
        IrType::EnumRef(ty) => {
            let en = ty.get(pack);
            en.variants.iter().flat_map(|variant| match variant.kind {
                IrVariantKind::Value => vec![],
                IrVariantKind::Struct(st) => handle_struct(
                    st,
                    format!("{}.{}", ty.ident.0.rust_style(), variant.name.rust_style()),
                ),
            })
        }
        _ => vec![],
    }
}

fn handle_struct(st: &IrStruct, partial_name: &str) -> Vec<TODO> {
    (st.fields.iter())
        .filter_map(|field| handle_field(field, partial_name))
        .collect()
}

fn handle_field(field: &IrField, partial_name: &str) -> Option<TODO> {
    if matches!(field.ty, IrType::RustAutoOpaqueImplicit(_)) {
        Some(TODO)
    } else {
        Noen
    }
}
