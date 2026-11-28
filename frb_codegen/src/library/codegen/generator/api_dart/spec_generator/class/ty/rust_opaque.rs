use crate::codegen::generator::api_dart::spec_generator::class::method::{
    generate_api_methods, GenerateApiMethodConfig, GenerateApiMethodMode, GeneratedApiMethods,
};
use crate::codegen::generator::api_dart::spec_generator::class::misc::generate_class_extra_body;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_imports_which_types_and_funcs_use;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::trait_impl::MirTraitImpl;
use crate::codegen::ir::mir::ty::rust_opaque::MirTypeRustOpaque;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use itertools::{concat, Itertools};

impl ApiDartGeneratorClassTrait for RustOpaqueApiDartGenerator<'_> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let Info {
            dart_api_type,
            methods,
        } = self.compute_info(
            &GenerateApiMethodConfig {
                mode_static: GenerateApiMethodMode::DeclAndImpl,
                mode_non_static: GenerateApiMethodMode::DeclOnly,
            },
            "",
        );
        let methods_str = &methods.code;

        let rust_api_type = self.mir.rust_api_type();

        let extra_code =
            generate_class_extra_body(self.mir_type(), &self.context.mir_pack.dart_code_of_type);
        let extra_body = &extra_code.body;
        let class_modifiers = if mixes_in_dart_code(&self.mir_type(), self.context) {
            "abstract mixin"
        } else {
            "abstract"
        };

        let (impl_code, impl_header) =
            generate_implements(&self.context.mir_pack.trait_impls, &self.mir, self.context);

        Some(ApiDartGeneratedClass {
            namespace: self.mir.namespace.clone(),
            class_name: dart_api_type.clone(),
            code: format!(
                "
                // Rust type: {rust_api_type}
                {class_modifiers} class {dart_api_type} implements {impl_code} {{
                    {methods_str}

                    {extra_body}
                }}
                "
            ),
            needs_freezed: false,
            needs_json_serializable: false,
            header: methods.header + impl_header + extra_code.header,
        })
    }

    fn generate_extra_impl_code(&self) -> Option<String> {
        let Info {
            dart_api_type,
            methods,
        } = self.compute_info(
            &GenerateApiMethodConfig {
                mode_static: GenerateApiMethodMode::Nothing,
                mode_non_static: GenerateApiMethodMode::DeclAndImpl,
            },
            "Impl",
        );
        let methods_str = &methods.code;

        let dart_api_type_impl = format!("{dart_api_type}Impl");
        let public_class_keyword = if mixes_in_dart_code(&self.mir_type(), self.context) {
            "with"
        } else {
            "implements"
        };

        let dart_entrypoint_class_name = &self.context.config.dart_entrypoint_class_name;
        let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

        Some(format!(
            "
            @sealed class {dart_api_type_impl} extends RustOpaque {public_class_keyword} {dart_api_type} {{
                // Not to be used by end users
                {dart_api_type_impl}.frbInternalDcoDecode(List<dynamic> wire):
                    super.frbInternalDcoDecode(wire, _kStaticData);

                // Not to be used by end users
                {dart_api_type_impl}.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative):
                    super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

                static final _kStaticData = RustArcStaticData(
                    rustArcIncrementStrongCount: {dart_api_instance}.rust_arc_increment_strong_count_{dart_api_type},
                    rustArcDecrementStrongCount: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type},
                    rustArcDecrementStrongCountPtr: {dart_api_instance}.rust_arc_decrement_strong_count_{dart_api_type}Ptr,
                );

                {methods_str}
            }}"
        ))
    }
}

impl RustOpaqueApiDartGenerator<'_> {
    fn compute_info(
        &self,
        config: &GenerateApiMethodConfig,
        dart_class_name_postfix: &str,
    ) -> Info {
        let dart_api_type = ApiDartGenerator::new(self.mir.clone(), self.context).dart_api_type();

        let methods = generate_api_methods(
            &MirType::RustOpaque(self.mir.clone()),
            self.context,
            config,
            &format!("{dart_api_type}{dart_class_name_postfix}"),
        );

        Info {
            dart_api_type,
            methods,
        }
    }
}

struct Info {
    dart_api_type: String,
    methods: GeneratedApiMethods,
}

/// Whether objects returned from Rust mix in the public class.
///
/// The public class, its `*Impl`, and proxy variants all use this. Mixing in requires Dart 3,
/// `dart_code_mixin` or `#[frb(dart_code_mixin)]`, and instance members in `dart_code`.
/// `#[frb(ui_state)]` never opts in. The parser stores `dart_code` on both the auto-opaque
/// wrapper and the inner opaque type; a borrowed proxy is resolved through the inner type.
pub(crate) fn mixes_in_dart_code(ty: &MirType, context: ApiDartGeneratorContext) -> bool {
    if !context.config.dart3 {
        return false;
    }
    let ty = match ty {
        MirType::RustAutoOpaqueImplicit(ty) => MirType::RustOpaque(ty.inner.clone()),
        ty => ty.clone(),
    };
    let ident = ty.safe_ident();
    if context.mir_pack.dart_code_mixin_blocked.contains(&ident) {
        return false;
    }
    let enabled =
        context.config.dart_code_mixin || context.mir_pack.dart_code_mixin_opt_in.contains(&ident);
    if !enabled {
        return false;
    }
    super::dart_code_body::has_instance_member(
        &generate_class_extra_body(ty, &context.mir_pack.dart_code_of_type).body,
    )
}

pub(crate) fn ensure_no_reserved_opaque_members(
    pack: &MirPack,
    config: &crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig,
) -> anyhow::Result<()> {
    if !config.dart3 {
        return Ok(());
    }
    for (key, code) in &pack.dart_code_of_type {
        if pack.dart_code_mixin_blocked.contains(key) {
            continue;
        }
        let opaque_key = key.starts_with("RustOpaque_") || key.starts_with("Auto_");
        let enabled =
            opaque_key && (config.dart_code_mixin || pack.dart_code_mixin_opt_in.contains(key));
        if !enabled || !super::dart_code_body::has_instance_member(&code.body) {
            continue;
        }
        if let Some(name) = super::dart_code_body::reserved_instance_member(&code.body) {
            anyhow::bail!(
                "`dart_code` on `{key}` declares `{name}`, which would override the member that owns the Rust object once the class is mixed in. Remove that declaration. Reserved names: {}.",
                super::dart_code_body::reserved_opaque_member_list()
            );
        }
    }
    Ok(())
}

fn generate_implements(
    all_trait_impls: &[MirTraitImpl],
    self_type: &MirTypeRustOpaque,
    context: ApiDartGeneratorContext,
) -> (String, DartHeaderCode) {
    let (names, header) = generate_implements_traits(all_trait_impls, self_type, context);
    (
        concat([vec!["RustOpaqueInterface".to_owned()], names]).join(", "),
        header,
    )
}

fn generate_implements_traits(
    all_trait_impls: &[MirTraitImpl],
    self_type: &MirTypeRustOpaque,
    context: ApiDartGeneratorContext,
) -> (Vec<String>, DartHeaderCode) {
    let interest_trait_impls = all_trait_impls
        .iter()
        .filter(|x| {
            (get_candidate_safe_idents_for_matching(&x.impl_ty).iter())
                .any(|x| x == &self_type.safe_ident())
        })
        .collect_vec();

    if interest_trait_impls.is_empty() {
        return (vec![], Default::default());
    }

    let impl_names = (interest_trait_impls.iter())
        .map(|t| ApiDartGenerator::new(t.trait_ty.clone(), context).dart_api_type())
        .collect_vec();

    let interest_trait_types = (interest_trait_impls.iter())
        .map(|x| MirType::TraitDef(x.trait_ty.clone()))
        .collect_vec();
    let import = generate_imports_which_types_and_funcs_use(
        &self_type.namespace,
        &Some(&interest_trait_types.iter().collect()),
        &None,
        context,
    )
    .unwrap();

    (
        impl_names,
        DartHeaderCode {
            import,
            ..Default::default()
        },
    )
}

#[allow(clippy::single_match)]
fn get_candidate_safe_idents_for_matching(ty: &MirType) -> Vec<String> {
    let mut ans = vec![ty.safe_ident()];
    match ty {
        MirType::RustAutoOpaqueImplicit(ty) => ans.push(ty.inner.safe_ident()),
        _ => {}
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::ir::mir::func::OwnershipMode;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::pack::MirPack;
    use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateProxyVariant};
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
        MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
    };
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::codegen::ir::mir::ty::MirTypeTrait;
    use crate::utils::basic_code::parser::parse_dart_code;
    use crate::utils::namespace::Namespace;
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;

    fn rust_opaque() -> MirTypeRustOpaque {
        MirTypeRustOpaque {
            namespace: Namespace::default(),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("Thing".into())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: None,
            brief_name: false,
        }
    }

    fn borrowed_auto_opaque() -> MirType {
        MirType::RustAutoOpaqueImplicit(MirTypeRustAutoOpaqueImplicit {
            ownership_mode: OwnershipMode::Ref,
            inner: rust_opaque(),
            raw: MirRustAutoOpaqueRaw {
                string: MirLifetimeAwareType::new("Thing".into()),
                segments: vec![],
            },
            reason: None,
            ignore: false,
        })
    }

    fn config(dart3: bool, dart_code_mixin: bool) -> GeneratorApiDartInternalConfig {
        GeneratorApiDartInternalConfig {
            dart_collection_deep_equality: false,
            dart_code_mixin,
            dart_enums_style: true,
            dart3,
            dart_decl_base_output_path: PathBuf::new(),
            dart_impl_output_path: Default::default(),
            dart_entrypoint_class_name: "RustLib".into(),
            dart_preamble: String::new(),
            dart_type_rename: HashMap::new(),
        }
    }

    /// The parser stores an opaque struct's `dart_code` under both its auto-opaque wrapper key
    /// and its inner opaque key. `extra_keys` adds those, on top of the inner key.
    fn pack(dart_code: Option<&str>, opt_in: bool, extra_keys: &[&str], blocked: bool) -> MirPack {
        let inner_key = MirType::RustOpaque(rust_opaque()).safe_ident();
        let mut dart_code_mixin_opt_in = HashSet::new();
        let mut dart_code_mixin_blocked = HashSet::new();
        if dart_code.is_some() {
            if opt_in {
                dart_code_mixin_opt_in.insert(inner_key.clone());
                for key in extra_keys {
                    dart_code_mixin_opt_in.insert((*key).to_owned());
                }
            }
            if blocked {
                dart_code_mixin_blocked.insert(inner_key.clone());
            }
        }
        MirPack {
            funcs_all: vec![],
            extra_types_all: vec![],
            struct_pool: Default::default(),
            enum_pool: Default::default(),
            dart_code_of_type: (dart_code.into_iter())
                .flat_map(|code| {
                    let parsed = parse_dart_code(code);
                    let mut keys = vec![inner_key.clone()];
                    keys.extend(extra_keys.iter().map(|key| (*key).to_owned()));
                    keys.into_iter().map(move |key| (key, parsed.clone()))
                })
                .collect(),
            dart_code_mixin_opt_in,
            dart_code_mixin_blocked,
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: Default::default(),
        }
    }

    /// Mixes in only for opted-in Dart 3 types whose `dart_code` has an instance member.
    #[test]
    fn mixes_in_dart_code_only_with_instance_members() {
        let ty = MirType::RustOpaque(rust_opaque());
        for (dart3, mixin, dart_code, expected) in [
            (true, true, None, false),
            (true, true, Some("import 'dart:math';\n"), false),
            (true, true, Some("// only a comment\n"), false),
            (
                true,
                true,
                Some("static int get dartCodeGetter => 123;"),
                false,
            ),
            (
                true,
                true,
                Some("factory Foo() {\n  return Foo.empty();\n}"),
                false,
            ),
            (true, true, Some("int extra() => 1;"), true),
            (true, false, Some("int extra() => 1;"), false),
            (false, true, Some("int extra() => 1;"), false),
        ] {
            let config = config(dart3, mixin);
            let pack = pack(dart_code, false, &[], false);
            let context = ApiDartGeneratorContext {
                mir_pack: &pack,
                config: &config,
            };
            assert_eq!(mixes_in_dart_code(&ty, context), expected);
        }
    }

    /// An item attribute opts in while the package flag stays off.
    #[test]
    fn item_opt_in_mixes_without_the_package_flag() {
        let ty = MirType::RustOpaque(rust_opaque());
        let config = config(true, false);
        let pack = pack(Some("int extra() => 1;"), true, &[], false);
        let context = ApiDartGeneratorContext {
            mir_pack: &pack,
            config: &config,
        };
        assert!(mixes_in_dart_code(&ty, context));
    }

    /// `#[frb(ui_state)]` does not mix in, even when the package flag is on.
    #[test]
    fn ui_state_does_not_mix_in() {
        let ty = MirType::RustOpaque(rust_opaque());
        let config = config(true, true);
        let pack = pack(Some("int extra() => 1;"), true, &[], true);
        let context = ApiDartGeneratorContext {
            mir_pack: &pack,
            config: &config,
        };
        assert!(!mixes_in_dart_code(&ty, context));
    }

    /// The parser records both keys. The inner-only map still resolves a borrowed proxy.
    #[test]
    fn mixes_in_dart_code_of_borrowed_auto_opaque_via_inner_type() {
        let config = config(true, false);
        let ty = borrowed_auto_opaque();
        let both = pack(Some("int extra() => 1;"), true, &[&ty.safe_ident()], false);
        assert!(both.dart_code_of_type.contains_key(&ty.safe_ident()));
        assert!(mixes_in_dart_code(
            &ty,
            ApiDartGeneratorContext {
                mir_pack: &both,
                config: &config,
            }
        ));

        let inner_only = pack(Some("int extra() => 1;"), true, &[], false);
        assert!(!inner_only.dart_code_of_type.contains_key(&ty.safe_ident()));
        assert!(mixes_in_dart_code(
            &ty,
            ApiDartGeneratorContext {
                mir_pack: &inner_only,
                config: &config,
            }
        ));
    }

    /// Rejects members that would shadow the Rust object's own disposal and move.
    #[test]
    fn rejects_reserved_opaque_members_when_mixing_in() {
        let without_dart3 = config(false, true);
        let config = config(true, false);
        let reserved = pack(Some("void dispose() {}"), true, &[], false);
        let error = ensure_no_reserved_opaque_members(&reserved, &config)
            .expect_err("reserved member")
            .to_string();
        assert!(error.contains("dispose"));
        assert!(error.contains("isDisposed"));

        assert!(ensure_no_reserved_opaque_members(&reserved, &without_dart3).is_ok());
        let blocked = pack(Some("void dispose() {}"), true, &[], true);
        assert!(ensure_no_reserved_opaque_members(&blocked, &config).is_ok());

        let allowed = pack(
            Some("// void dispose()\nint extra() => 1;"),
            true,
            &[],
            false,
        );
        assert!(ensure_no_reserved_opaque_members(&allowed, &config).is_ok());
    }

    /// Keeps the public class, its `Impl` and proxy variants consistent about mixing in.
    #[test]
    fn generates_mixin_supertypes_only_with_dart_code_members() {
        let proxy = MirType::Delegate(MirTypeDelegate::ProxyVariant(MirTypeDelegateProxyVariant {
            inner: Box::new(borrowed_auto_opaque()),
            upstream: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
            upstream_method_name: "borrow".into(),
        }));
        let generate = |dart3, mixin, dart_code| {
            let config = config(dart3, mixin);
            let pack = pack(dart_code, mixin, &[], false);
            let context = ApiDartGeneratorContext {
                mir_pack: &pack,
                config: &config,
            };
            let opaque = ApiDartGenerator::new(rust_opaque(), context);
            (
                opaque.generate_class().unwrap().code,
                opaque.generate_extra_impl_code().unwrap(),
                (ApiDartGenerator::new(proxy.clone(), context).generate_extra_impl_code()).unwrap(),
            )
        };

        let (class, impl_class, proxy_class) = generate(true, true, Some("int extra() => 1;"));
        assert!(class.contains("abstract mixin class Thing implements RustOpaqueInterface {"));
        assert!(impl_class.contains("class ThingImpl extends RustOpaque with Thing {"));
        assert!(proxy_class.contains("with SimpleDisposable, Thing {"));

        for (dart3, mixin, dart_code) in [
            (true, true, None),
            (true, true, Some("static int get onlyStatic => 1;")),
            (true, false, Some("int extra() => 1;")),
            (false, true, Some("int extra() => 1;")),
        ] {
            let (class, impl_class, proxy_class) = generate(dart3, mixin, dart_code);
            assert!(class.contains("abstract class Thing implements RustOpaqueInterface {"));
            assert!(impl_class.contains("class ThingImpl extends RustOpaque implements Thing {"));
            assert!(proxy_class.contains("with SimpleDisposable implements Thing {"));
        }
    }

    /// Includes both the wrapper and inner identifiers for implicit auto opaque types.
    #[test]
    fn candidate_idents_include_implicit_wrapper_and_inner_type() {
        let inner = rust_opaque();
        let ty = MirType::RustAutoOpaqueImplicit(MirTypeRustAutoOpaqueImplicit {
            ownership_mode: OwnershipMode::Owned,
            inner: inner.clone(),
            raw: MirRustAutoOpaqueRaw {
                string: MirLifetimeAwareType::new("Thing".into()),
                segments: vec![],
            },
            reason: None,
            ignore: false,
        });

        assert_eq!(
            get_candidate_safe_idents_for_matching(&ty),
            vec![ty.safe_ident(), inner.safe_ident()]
        );
    }

    /// Keeps ordinary types to their sole safe identifier.
    #[test]
    fn candidate_idents_leave_non_implicit_types_unchanged() {
        let ty = MirType::RustOpaque(rust_opaque());
        assert_eq!(
            get_candidate_safe_idents_for_matching(&ty),
            vec![ty.safe_ident()]
        );
    }
}
