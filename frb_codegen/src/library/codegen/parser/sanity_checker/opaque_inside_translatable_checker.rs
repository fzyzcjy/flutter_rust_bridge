use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::enumeration::MirVariantKind;
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::codegen::ir::mir::ty::MirType;
use itertools::Itertools;
use log::info;

pub(crate) fn check_opaque_inside_translatable(pack: &MirPack) {
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

fn handle_type(pack: &MirPack, ty: MirType) -> Vec<String> {
    match ty {
        MirType::StructRef(ty) => {
            let st = ty.get(pack);
            handle_struct(st, &ty.ident.0.rust_style())
        }
        MirType::EnumRef(ty) => {
            let en = ty.get(pack);
            en.variants
                .iter()
                .flat_map(|variant| match &variant.kind {
                    MirVariantKind::Value => vec![],
                    MirVariantKind::Struct(st) => handle_struct(
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

fn handle_struct(st: &MirStruct, partial_name: &str) -> Vec<String> {
    (st.fields.iter())
        .filter_map(|field| handle_field(field, partial_name))
        .collect()
}

fn handle_field(field: &MirField, partial_name: &str) -> Option<String> {
    if matches!(field.ty, MirType::RustAutoOpaqueImplicit(_)) {
        Some(format!("{partial_name}.{}", field.name.rust_style()))
    } else {
        None
    }
}
