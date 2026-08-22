use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::ir::mir::func::MirFuncMode;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegateProxyVariant;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};

pub(crate) fn compute_func_implementation(
    ir: &MirTypeDelegateProxyVariant,
    context: ApiDartGeneratorContext,
    func_mode: MirFuncMode,
) -> String {
    let mut ans = format!("{}(this)", compute_dart_extra_type(ir, context));
    if func_mode == MirFuncMode::Normal {
        ans = format!("Future.value({ans})");
    }
    ans
}

pub(crate) fn compute_dart_extra_type(
    ir: &MirTypeDelegateProxyVariant,
    context: ApiDartGeneratorContext,
) -> String {
    let inner_dart_api_type = ApiDartGenerator::new(ir.inner.clone(), context).dart_api_type();
    let upstream_dart_api_type =
        ApiDartGenerator::new(ir.upstream.clone(), context).dart_api_type();
    format!(
        "{}ProxyVariant{}{}",
        inner_dart_api_type,
        upstream_dart_api_type,
        ir.upstream_method_name.to_case(Case::Pascal),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::ir::mir::pack::MirPack;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn config() -> GeneratorApiDartInternalConfig {
        GeneratorApiDartInternalConfig {
            dart_collection_deep_equality: false,
            dart_enums_style: true,
            dart3: true,
            dart_decl_base_output_path: PathBuf::new(),
            dart_impl_output_path: Default::default(),
            dart_entrypoint_class_name: "Entrypoint".into(),
            dart_preamble: String::new(),
            dart_type_rename: HashMap::new(),
        }
    }

    fn pack() -> MirPack {
        MirPack {
            funcs_all: vec![],
            extra_types_all: vec![],
            struct_pool: Default::default(),
            enum_pool: Default::default(),
            dart_code_of_type: Default::default(),
            existing_handler: None,
            skips: vec![],
            trait_impls: vec![],
            extra_rust_output_code: String::new(),
            extra_dart_output_code: Default::default(),
        }
    }

    #[test]
    /// Wraps normal proxy constructors in Future.value while leaving sync constructors direct.
    fn wraps_only_normal_proxy_variant_implementations_in_a_future() {
        let config = config();
        let mir_pack = pack();
        let context = ApiDartGeneratorContext {
            mir_pack: &mir_pack,
            config: &config,
        };
        let proxy = MirTypeDelegateProxyVariant {
            inner: Box::new(crate::codegen::ir::mir::ty::MirType::Primitive(
                MirTypePrimitive::I32,
            )),
            upstream: Box::new(crate::codegen::ir::mir::ty::MirType::Primitive(
                MirTypePrimitive::Bool,
            )),
            upstream_method_name: "from_value".into(),
        };

        assert_eq!(
            compute_func_implementation(&proxy, context, MirFuncMode::Sync),
            "intProxyVariantboolFromValue(this)"
        );
        assert_eq!(
            compute_func_implementation(&proxy, context, MirFuncMode::Normal),
            "Future.value(intProxyVariantboolFromValue(this))"
        );
    }
}
