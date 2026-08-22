use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl CodecSseTyTrait for GeneralListCodecSseTy<'_> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(general_list_generate_encode(lang, &self.mir.inner))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(general_list_generate_decode(
            lang,
            &self.mir.inner,
            self.context,
        ))
    }
}

pub(super) fn general_list_generate_encode(lang: &Lang, mir_inner: &MirType) -> String {
    format!(
        "{};
        {}",
        lang.call_encode(&LEN_TYPE, &format!("self.{}", list_len_method(lang))),
        lang.for_loop(
            "item",
            "self",
            &format!("{};", lang.call_encode(mir_inner, "item")),
        )
    )
}

pub(super) fn list_len_method(lang: &Lang) -> &'static str {
    match lang {
        Lang::DartLang(_) => "length",
        Lang::RustLang(_) => "len() as _",
    }
}

pub(super) fn general_list_generate_decode(
    lang: &Lang,
    mir_inner: &MirType,
    context: CodecSseTyContext,
) -> String {
    let var_decl = lang.var_decl();

    let init = match lang {
        Lang::DartLang(_) => format!(
            "<{}>[]",
            ApiDartGenerator::new(mir_inner.clone(), context.as_api_dart_context()).dart_api_type()
        ),
        Lang::RustLang(_) => "Vec::with_capacity(len_ as usize)".to_owned(),
    };
    let list_push = match lang {
        Lang::DartLang(_) => "add",
        Lang::RustLang(_) => "push",
    };

    format!(
        "
        {var_decl} len_ = {};
        {var_decl} ans_ = {init};
        {}
        return ans_;
        ",
        lang.call_decode(&LEN_TYPE),
        lang.for_range_loop(
            "idx_",
            "len_",
            &format!("ans_.{list_push}({});", lang.call_decode(mir_inner))
        ),
    )
}

pub(super) const LEN_TYPE: MirType = Primitive(MirTypePrimitive::I32);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::codec::sse::lang::{dart::DartLang, rust::RustLang};
    use std::collections::HashMap;
    use std::path::PathBuf;

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

    /// Uses the target language's native list length expression.
    #[test]
    fn list_length_method_matches_target_language_syntax() {
        assert_eq!(list_len_method(&Lang::DartLang(DartLang)), "length");
        assert_eq!(list_len_method(&Lang::RustLang(RustLang)), "len() as _");
    }

    /// Generates target-local length, iteration, and primitive inner codec calls.
    #[test]
    fn general_list_encode_uses_target_specific_length_and_loop_syntax() {
        let inner = Primitive(MirTypePrimitive::I32);
        let dart = general_list_generate_encode(&Lang::DartLang(DartLang), &inner);
        assert!(dart.contains("self.length"));
        assert!(dart.contains("for (final item in self)"));
        assert!(dart.contains("sse_encode_i_32(item, serializer)"));

        let rust = general_list_generate_encode(&Lang::RustLang(RustLang), &inner);
        assert!(rust.contains("self.len() as _"));
        assert!(rust.contains("for item in self"));
        assert!(rust.contains("<i32>::sse_encode(item, serializer)"));
    }

    /// Allocates and appends decoded values with each target's list API.
    #[test]
    fn general_list_decode_uses_target_specific_allocation_and_append() {
        let pack = pack();
        let config = config();
        let inner = Primitive(MirTypePrimitive::I32);
        let context = CodecSseTyContext::new(&pack, &config);
        let dart = general_list_generate_decode(&Lang::DartLang(DartLang), &inner, context);
        assert!(dart.contains("<int>[]"));
        assert!(dart.contains("ans_.add(sse_decode_i_32(deserializer))"));

        let rust = general_list_generate_decode(&Lang::RustLang(RustLang), &inner, context);
        assert!(rust.contains("Vec::with_capacity(len_ as usize)"));
        assert!(rust.contains("ans_.push(<i32>::sse_decode(deserializer))"));
    }
}
