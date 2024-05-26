use crate::codegen::ir::field::IrField;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;
use log::info;

pub(crate) fn check_opaque_inside_translatable(pack: &IrPack) {
    let hint_names = (pack.distinct_types(None).into_iter())
        .flat_map(|ty| handle_type(pack, ty))
        .collect_vec();
    if !hint_names.is_empty() {
        info!(
            "It is suggested (but not required) to wrap opaque inside non-opaque type with `RustAutoOpaque<..>`. \
            See https://fzyzcjy.github.io/flutter_rust_bridge/guides/types/arbitrary/rust-auto-opaque/opaque-in-translatable for more details. \
            Related types: {}",
            hint_names.join(", "),
        );
    }
}

fn handle_type(pack: &IrPack, ty: IrType) -> Vec<String> {
    match ty {
        IrType::StructRef(ty) => {
            let st = ty.get(pack);
            handle_struct(st, &ty.ident.0.rust_style())
        }
        IrType::EnumRef(ty) => {
            let en = ty.get(pack);
            en.variants
                .iter()
                .flat_map(|variant| match &variant.kind {
                    IrVariantKind::Value => vec![],
                    IrVariantKind::Struct(st) => handle_struct(
                        st,
                        &format!("{}.{}", ty.ident.0.rust_style(), variant.name.rust_style()),
                    ),
                })
                .collect_vec()
        }
        // TODO also check and hint `Vec<OpaqueType>`, etc
        _ => vec![],
    }
}

fn handle_struct(st: &IrStruct, partial_name: &str) -> Vec<String> {
    (st.fields.iter())
        .filter_map(|field| handle_field(field, partial_name))
        .collect()
}

fn handle_field(field: &IrField, partial_name: &str) -> Option<String> {
    if matches!(field.ty, IrType::RustAutoOpaqueImplicit(_)) {
        Some(format!("{partial_name}.{}", field.name.rust_style()))
    } else {
        None
    }
}
