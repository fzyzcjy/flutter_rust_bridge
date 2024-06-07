// use crate::codegen::ir::mir::field::MirField;
// use crate::codegen::ir::mir::ident::MirIdent;
// use crate::codegen::ir::mir::pack::MirPack;
// use crate::codegen::ir::mir::ty::delegate::{
//     MirTypeDelegate, MirTypeDelegateDynTrait, MirTypeDelegateRustAutoOpaqueExplicit,
// };
// use crate::codegen::ir::mir::ty::enumeration::{
//     MirEnum, MirEnumIdent, MirEnumMode, MirEnumVariant, MirVariantKind,
// };
// use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
//     MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
// };
// use crate::codegen::ir::mir::ty::rust_opaque::MirTypeRustOpaque;
// use crate::codegen::ir::mir::ty::structure::MirStruct;
// use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};
// use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
// use crate::codegen::parser::mir::parser::ty::enumeration::compute_enum_variant_kind_struct_name;
// use crate::codegen::parser::mir::parser::ty::rust_auto_opaque_implicit::parse_type_rust_auto_opaque_common_raw;
// use crate::if_then_some;
// use crate::utils::namespace::NamespacedName;
// use itertools::Itertools;
//
// pub(crate) fn transform(
//     mut pack: MirPack,
// ) -> anyhow::Result<MirPack> {
//     let distinct_types = pack.distinct_types(None);
//
//     let ty_dyn_traits = (distinct_types.iter())
//         .filter_map(
//             |ty| if_then_some!(let MirType::Delegate(MirTypeDelegate::DynTrait(ty)) = ty , ty.clone()),
//         )
//         .unique_by(|ty| ty.safe_ident())
//         .collect_vec();
//
//     for ty_dyn_trait in &ty_dyn_traits {
//         handle_ty_dyn_trait(&mut pack, ty_dyn_trait)?;
//     }
//
//     Ok(pack)
// }
//
// fn handle_ty_dyn_trait(
//     pack: &mut MirPack,
//     ty_dyn_trait: &MirTypeDelegateDynTrait,
// ) -> anyhow::Result<()> {
//     let interest_impl_types = (pack.trait_impls.iter())
//         .filter(|item| item.trait_ty.name == ty_dyn_trait.trait_def_name)
//         .map(|item| item.impl_ty.clone())
//         .filter_map(|ty| if_then_some!(let MirType::RustAutoOpaqueImplicit(ty) = ty, ty))
//         .collect_vec();
//     let enum_name = ty_dyn_trait.inner_raw().ident;
//     let mir_enum = create_enum(&interest_impl_types, &enum_name.0)?;
//
//     pack.enum_pool.insert(enum_name, mir_enum);
//
//     Ok(())
// }
//
// fn create_enum(
//     interest_impl_types: &[MirTypeRustAutoOpaqueImplicit],
//     enum_name: &NamespacedName,
// ) -> anyhow::Result<MirEnum> {
//     let variants = (interest_impl_types.iter())
//         .map(|ty| create_enum_variant(ty, enum_name))
//         .collect::<anyhow::Result<Vec<_>>>()?;
//
//     Ok(MirEnum {
//         name: enum_name.clone(),
//         wrapper_name: None,
//         comments: vec![],
//         variants,
//         mode: MirEnumMode::Complex,
//         ignore: false,
//     })
// }
//
// fn create_enum_variant(
//     interest_ty: &MirTypeRustAutoOpaqueImplicit,
//     enum_name: &NamespacedName,
// ) -> anyhow::Result<MirEnumVariant> {
//     let variant_name = MirIdent::new(interest_ty.safe_ident());
//
//     let field_ty = MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(
//         MirTypeDelegateRustAutoOpaqueExplicit {
//             raw: interest_ty.raw.clone(),
//             inner: interest_ty.inner.clone(),
//         },
//     ));
//
//     Ok(MirEnumVariant {
//         name: variant_name.clone(),
//         wrapper_name: MirIdent::new(format!("{}_{}", enum_name.name, variant_name.raw)),
//         comments: vec![],
//         kind: MirVariantKind::Struct(MirStruct {
//             name: compute_enum_variant_kind_struct_name(enum_name, &variant_name),
//             wrapper_name: None,
//             fields: vec![MirField {
//                 ty: field_ty,
//                 name: MirIdent::new("value".to_string()),
//                 is_final: false,
//                 is_rust_public: None,
//                 comments: vec![],
//                 default: None,
//                 settings: Default::default(),
//             }],
//             is_fields_named: false,
//             dart_metadata: vec![],
//             ignore: false,
//             generate_hash: false,
//             generate_eq: false,
//             comments: vec![],
//         }),
//     })
// }
