use crate::codegen::generator::api_dart::spec_generator::class::method::{
    generate_api_methods, GenerateApiMethodConfig, GenerateApiMethodMode, GeneratedApiMethods,
};
use crate::codegen::generator::api_dart::spec_generator::class::misc::generate_class_extra_body;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_imports_which_types_and_funcs_use;
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

/// Classes of objects returned from Rust only inherit `dart_code` members by mixing in the
/// public class, which is a mixin class exactly when this returns true.
pub(crate) fn mixes_in_dart_code(ty: &MirType, context: ApiDartGeneratorContext) -> bool {
    // Mixin classes are a Dart 3 language feature.
    if !context.config.dart3 {
        return false;
    }
    // Borrowed auto opaque types (e.g. proxies) are not keys of `dart_code_of_type`, but their
    // inner opaque type is.
    let ty = match ty {
        MirType::RustAutoOpaqueImplicit(ty) => MirType::RustOpaque(ty.inner.clone()),
        ty => ty.clone(),
    };
    !generate_class_extra_body(ty, &context.mir_pack.dart_code_of_type)
        .body
        .trim()
        .is_empty()
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
    use std::collections::HashMap;
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

    fn config(dart3: bool) -> GeneratorApiDartInternalConfig {
        GeneratorApiDartInternalConfig {
            dart_collection_deep_equality: false,
            dart_enums_style: true,
            dart3,
            dart_decl_base_output_path: PathBuf::new(),
            dart_impl_output_path: Default::default(),
            dart_entrypoint_class_name: "RustLib".into(),
            dart_preamble: String::new(),
            dart_type_rename: HashMap::new(),
        }
    }

    /// Mirrors the parser, which keys the `dart_code` of an opaque struct by its inner opaque type.
    fn pack(dart_code: Option<&str>) -> MirPack {
        MirPack {
            funcs_all: vec![],
            extra_types_all: vec![],
            struct_pool: Default::default(),
            enum_pool: Default::default(),
            dart_code_of_type: (dart_code.into_iter())
                .map(|code| {
                    let key = MirType::RustOpaque(rust_opaque()).safe_ident();
                    (key, parse_dart_code(code))
                })
                .collect(),
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: Default::default(),
        }
    }

    /// Mixes in only when `dart_code` has members (imports alone leave nothing to inherit) and
    /// Dart 3 features, which mixin classes need, are enabled.
    #[test]
    fn mixes_in_dart_code_only_with_members() {
        let ty = MirType::RustOpaque(rust_opaque());
        for (dart3, dart_code, expected) in [
            (true, None, false),
            (true, Some("import 'dart:math';\n"), false),
            (true, Some("int extra() => 1;"), true),
            (false, Some("int extra() => 1;"), false),
        ] {
            let config = config(dart3);
            let pack = pack(dart_code);
            let context = ApiDartGeneratorContext {
                mir_pack: &pack,
                config: &config,
            };
            assert_eq!(
                mixes_in_dart_code(&ty, context),
                expected,
                "{dart3} {dart_code:?}"
            );
        }
    }

    /// Finds the `dart_code` of a borrowed auto opaque type, which is not a key itself.
    #[test]
    fn mixes_in_dart_code_of_borrowed_auto_opaque_via_inner_type() {
        let config = config(true);
        let pack = pack(Some("int extra() => 1;"));
        let context = ApiDartGeneratorContext {
            mir_pack: &pack,
            config: &config,
        };
        let ty = borrowed_auto_opaque();

        assert!(!pack.dart_code_of_type.contains_key(&ty.safe_ident()));
        assert!(mixes_in_dart_code(&ty, context));
    }

    /// Keeps the public class, its `Impl` and proxy variants consistent about mixing in.
    #[test]
    fn generates_mixin_supertypes_only_with_dart_code_members() {
        let proxy = MirType::Delegate(MirTypeDelegate::ProxyVariant(MirTypeDelegateProxyVariant {
            inner: Box::new(borrowed_auto_opaque()),
            upstream: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
            upstream_method_name: "borrow".into(),
        }));
        let generate = |dart3, dart_code| {
            let config = config(dart3);
            let pack = pack(dart_code);
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

        let (class, impl_class, proxy_class) = generate(true, Some("int extra() => 1;"));
        assert!(class.contains("abstract mixin class Thing implements RustOpaqueInterface {"));
        assert!(impl_class.contains("class ThingImpl extends RustOpaque with Thing {"));
        assert!(proxy_class.contains("with SimpleDisposable, Thing {"));

        for (dart3, dart_code) in [(true, None), (false, Some("int extra() => 1;"))] {
            let (class, impl_class, proxy_class) = generate(dart3, dart_code);
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
