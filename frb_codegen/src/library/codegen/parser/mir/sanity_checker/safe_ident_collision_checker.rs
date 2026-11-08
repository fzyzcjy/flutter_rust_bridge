use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};
use anyhow::bail;
use itertools::Itertools;
use log::warn;
use std::collections::HashMap;

const SKIP_ENV_VAR: &str = "FRB_DEBUG_SKIP_SANITY_CHECK_SAFE_IDENT_COLLISIONS";

/// Generated code is keyed by `safe_ident` (e.g. `sse_encode_unit`), so two different types
/// sharing one would make one of them silently disappear (e.g. a user `enum Unit` vs `()`).
pub(crate) fn check_safe_ident_collisions(pack: &MirPack) -> anyhow::Result<()> {
    let collisions = find_collisions(pack);
    if collisions.is_empty() {
        return Ok(());
    }

    let involves_primitive = (collisions.iter())
        .flat_map(|(_, types)| types)
        .any(|ty| matches!(ty, MirType::Primitive(_)));
    let message = format!(
        "Different types map to the same generated identifier, so one of them would silently \
        replace the other in the generated code: {}. Please rename one of the types{}.",
        collisions
            .iter()
            .map(|(ident, types)| format!(
                "{} map to `{ident}`",
                types
                    .iter()
                    .map(|ty| format!("`{}`", ty.rust_api_type()))
                    .join(" and ")
            ))
            .join("; "),
        if involves_primitive {
            " (names such as `Unit`, `Bool`, `U8`, `I32`, `F64` or `Usize` clash with built-in primitive types)"
        } else {
            ""
        },
    );

    // Setting env vars is racy in parallel tests, so the escape hatch is not tested
    // frb-coverage:ignore-start
    if std::env::var(SKIP_ENV_VAR).is_ok() {
        warn!("{message}");
        return Ok(());
    }
    // frb-coverage:ignore-end
    bail!(
        "{message} Another way to debug is to temporarily set environment variable `{SKIP_ENV_VAR}=1` and check the generated code."
    )
}

fn find_collisions(pack: &MirPack) -> Vec<(String, Vec<MirType>)> {
    let mut types_of_ident: HashMap<String, Vec<MirType>> = HashMap::new();
    pack.visit_types(
        &mut |ty| {
            let types = types_of_ident.entry(ty.safe_ident()).or_default();
            let seen = !types.is_empty();
            // The same type may be visited with different usage flags (e.g. as an error), or be
            // spelled with different whitespace (e.g. `Foo<Bar>` vs `Foo< Bar>`), which is fine
            let rust_type = strip_whitespace(&ty.rust_api_type());
            if !(types.iter()).any(|x| strip_whitespace(&x.rust_api_type()) == rust_type) {
                types.push(ty.clone());
            }
            seen
        },
        &None,
    );

    types_of_ident
        .into_iter()
        .filter(|(_, types)| types.len() > 1)
        // make the message independent of the visiting order
        .map(|(ident, types)| {
            (
                ident,
                types
                    .into_iter()
                    .sorted_by_cached_key(|ty| ty.rust_api_type())
                    .collect(),
            )
        })
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect()
}

fn strip_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
    use crate::codegen::ir::mir::extra_type::MirExtraType;
    use crate::codegen::ir::mir::func::OwnershipMode;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::enumeration::{
        MirEnum, MirEnumIdent, MirEnumMode, MirTypeEnumRef,
    };
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
        MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
    };
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent, MirTypeStructRef};
    use crate::utils::basic_code::general_code::GeneralDartCode;
    use crate::utils::namespace::{Namespace, NamespacedName};

    fn pack(types: Vec<MirType>) -> MirPack {
        let mut struct_pool = HashMap::new();
        let mut enum_pool = HashMap::new();
        for ty in &types {
            match ty {
                MirType::StructRef(inner) => {
                    struct_pool.insert(inner.ident.clone(), empty_struct(inner.ident.0.clone()));
                }
                MirType::EnumRef(inner) => {
                    enum_pool.insert(inner.ident.clone(), empty_enum(inner.ident.0.clone()));
                }
                _ => {}
            }
        }

        MirPack {
            funcs_all: vec![],
            extra_types_all: (types.into_iter())
                .map(|ty| MirExtraType {
                    ty,
                    codec_mode_pack: CodecModePack {
                        dart2rust: CodecMode::Sse,
                        rust2dart: CodecMode::Sse,
                    },
                })
                .collect(),
            struct_pool,
            enum_pool,
            dart_code_of_type: HashMap::new(),
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: GeneralDartCode::default(),
        }
    }

    fn empty_struct(name: NamespacedName) -> MirStruct {
        MirStruct {
            name,
            wrapper_name: None,
            fields: vec![],
            is_fields_named: true,
            dart_metadata_raw: vec![],
            ignore: false,
            needs_json_serializable: false,
            generate_hash: false,
            generate_eq: false,
            dart_collection_deep_equality: false,
            ui_state: false,
            comments: vec![],
        }
    }

    fn empty_enum(name: NamespacedName) -> MirEnum {
        MirEnum {
            name,
            wrapper_name: None,
            comments: vec![],
            variants: vec![],
            mode: MirEnumMode::Simple,
            ignore: false,
            needs_json_serializable: false,
        }
    }

    fn name(namespace: &str, name: &str) -> NamespacedName {
        NamespacedName::new(Namespace::new_raw(namespace.to_owned()), name.to_owned())
    }

    fn enum_ref(namespace: &str, ident: &str) -> MirType {
        MirType::EnumRef(MirTypeEnumRef {
            ident: MirEnumIdent(name(namespace, ident)),
            is_exception: false,
        })
    }

    fn struct_ref(namespace: &str, ident: &str, is_exception: bool) -> MirType {
        MirType::StructRef(MirTypeStructRef {
            ident: MirStructIdent(name(namespace, ident)),
            is_exception,
        })
    }

    fn auto_opaque(raw: &str) -> MirType {
        MirType::RustAutoOpaqueImplicit(MirTypeRustAutoOpaqueImplicit {
            ownership_mode: OwnershipMode::Owned,
            inner: MirTypeRustOpaque {
                namespace: Namespace::default(),
                inner: MirRustOpaqueInner(MirLifetimeAwareType::new("Foo<Bar>".into())),
                codec: RustOpaqueCodecMode::Moi,
                dart_api_type: None,
                brief_name: false,
            },
            raw: MirRustAutoOpaqueRaw {
                string: MirLifetimeAwareType::new(raw.into()),
                segments: vec![],
            },
            reason: None,
            ignore: false,
        })
    }

    fn collided_rust_types(pack: &MirPack) -> Vec<(String, Vec<String>)> {
        (find_collisions(pack).into_iter())
            .map(|(ident, types)| (ident, types.iter().map(|ty| ty.rust_api_type()).collect()))
            .collect()
    }

    /// A user enum named `Unit` clashes with the primitive `()`.
    #[test]
    fn detects_user_type_clashing_with_primitive() {
        let pack = pack(vec![
            MirType::Primitive(MirTypePrimitive::Unit),
            enum_ref("crate::api::minimal", "Unit"),
        ]);

        assert_eq!(
            collided_rust_types(&pack),
            vec![(
                "unit".to_owned(),
                vec!["()".to_owned(), "crate::api::minimal::Unit".to_owned()]
            )]
        );
        let message = check_safe_ident_collisions(&pack).unwrap_err().to_string();
        assert!(message.contains("`()` and `crate::api::minimal::Unit` map to `unit`"));
        assert!(message.contains("clash with built-in primitive types"));
        assert!(message.contains(SKIP_ENV_VAR));
    }

    /// Lists every collision in a stable order, and only hints primitives when one is involved.
    #[test]
    fn reports_all_collisions() {
        let message = check_safe_ident_collisions(&pack(vec![
            enum_ref("crate::b", "Foo"),
            MirType::Primitive(MirTypePrimitive::Unit),
            struct_ref("crate::a", "Foo", false),
            enum_ref("crate::api", "Unit"),
        ]))
        .unwrap_err()
        .to_string();
        assert!(message.contains(
            "`crate::a::Foo` and `crate::b::Foo` map to `foo`; `()` and `crate::api::Unit` map to `unit`."
        ));

        let message = check_safe_ident_collisions(&pack(vec![
            enum_ref("crate::b", "Foo"),
            struct_ref("crate::a", "Foo", false),
        ]))
        .unwrap_err()
        .to_string();
        assert!(!message.contains("primitive"));
    }

    /// The same type used both normally and as an error is not a clash.
    #[test]
    fn allows_same_type_with_different_usage() {
        let pack = pack(vec![
            struct_ref("crate::api", "MyError", false),
            struct_ref("crate::api", "MyError", true),
            enum_ref("crate::api", "Other"),
            MirType::Primitive(MirTypePrimitive::Unit),
        ]);

        assert!(find_collisions(&pack).is_empty());
        assert!(check_safe_ident_collisions(&pack).is_ok());
    }

    /// The same opaque type spelled with different whitespace is not a clash.
    #[test]
    fn allows_same_type_with_different_whitespace() {
        let pack = pack(vec![auto_opaque("Foo<Bar>"), auto_opaque("Foo< Bar>")]);

        assert!(find_collisions(&pack).is_empty());
    }
}
