use crate::codegen::generator::api_dart::spec_generator::class::method::dart_constructor_postfix;
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::misc::struct_or_record::StructOrRecord;
use crate::codegen::generator::misc::struct_or_record::StructOrRecord::Struct;
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use itertools::Itertools;

impl CodecSseTyTrait for StructRefCodecSseTy<'_> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(self.new_generalized_generator().generate_encode(lang))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(
            self.new_generalized_generator()
                .generate_decode(lang, None, true),
        )
    }
}

impl StructRefCodecSseTy<'_> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator<'_> {
        GeneralizedStructGenerator::new(
            self.mir.get(self.context.mir_pack).clone(),
            self.context,
            Struct,
        )
    }
}

pub(crate) struct GeneralizedStructGenerator<'a> {
    st: MirStruct,
    mode: StructOrRecord,
    context: CodecSseTyContext<'a>,
}

impl<'a> GeneralizedStructGenerator<'a> {
    pub(crate) fn new(st: MirStruct, context: CodecSseTyContext<'a>, mode: StructOrRecord) -> Self {
        Self { st, mode, context }
    }

    pub(super) fn generate_encode(&self, lang: &Lang) -> String {
        (self.st.fields.iter().enumerate())
            .map(|(index, field)| {
                format!(
                    "{};\n",
                    lang.call_encode(
                        &field.ty,
                        &format!(
                            "self.{}",
                            self.mode
                                .field_name(index, field, self.st.is_fields_named, lang)
                        )
                    )
                )
            })
            .join("")
    }

    pub(super) fn generate_decode(
        &self,
        lang: &Lang,
        override_struct_name: Option<String>,
        dart_unconditionally_kwargs_ctor: bool,
    ) -> String {
        let decode_fields = (self.st.fields.iter())
            .map(|field| {
                format!(
                    "{} var_{} = {};\n",
                    lang.var_decl(),
                    field.name.dart_style(),
                    lang.call_decode(&field.ty)
                )
            })
            .join("");

        let ctor = match self.mode {
            Struct => lang.call_constructor(
                &override_struct_name.unwrap_or_else(|| self.st.name.style(lang)),
                dart_constructor_postfix(
                    &self.st.name.name,
                    &self.context.mir_pack.funcs_with_impl(),
                    self.context.as_api_dart_context(),
                ),
                &(self.st.fields.iter())
                    .map(|x| x.name.style(lang, false))
                    .collect_vec(),
                &(self.st.fields.iter())
                    .map(|x| format!("var_{}", x.name.dart_style().clone()))
                    .collect_vec(),
                (matches!(lang, Lang::DartLang(_)) && dart_unconditionally_kwargs_ctor)
                    || self.st.is_fields_named,
            ),
            StructOrRecord::Record => format!(
                "({})",
                (self.st.fields.iter())
                    .map(|x| format!("var_{}", x.name.dart_style().clone()))
                    .join(", ")
            ),
        };

        format!("{decode_fields}return {ctor};")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
    use crate::codegen::generator::codec::sse::lang::{dart::DartLang, rust::RustLang};
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::utils::namespace::{Namespace, NamespacedName};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn field(name: &str, ty: MirType) -> MirField {
        MirField {
            ty,
            name: MirIdent::new(name.into(), None),
            is_final: false,
            is_rust_public: None,
            comments: vec![],
            default: None,
            settings: MirFieldSettings::default(),
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
    fn structure() -> MirStruct {
        MirStruct {
            name: NamespacedName::new(Namespace::new_raw("crate".into()), "Pair".into()),
            wrapper_name: None,
            fields: vec![
                field("first", Primitive(MirTypePrimitive::I32)),
                field("second", Primitive(MirTypePrimitive::Bool)),
            ],
            is_fields_named: false,
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

    /// Encodes structure fields in declaration order for Rust output.
    #[test]
    fn generalized_structure_encode_preserves_field_declaration_order() {
        let pack = pack();
        let config = config();
        let generator = GeneralizedStructGenerator::new(
            structure(),
            CodecSseTyContext::new(&pack, &config),
            Struct,
        );
        let output = generator.generate_encode(&Lang::RustLang(RustLang));
        assert!(output.find("self.0").unwrap() < output.find("self.1").unwrap());
        assert!(output.contains("<i32>::sse_encode(self.0, serializer)"));
        assert!(output.contains("<bool>::sse_encode(self.1, serializer)"));
    }

    /// Decodes record fields into a positional tuple constructor.
    #[test]
    fn generalized_record_decode_uses_positional_constructor() {
        let pack = pack();
        let config = config();
        let generator = GeneralizedStructGenerator::new(
            structure(),
            CodecSseTyContext::new(&pack, &config),
            StructOrRecord::Record,
        );
        let output = generator.generate_decode(&Lang::DartLang(DartLang), None, true);
        assert!(output.contains("var var_first = sse_decode_i_32(deserializer)"));
        assert!(output.contains("var var_second = sse_decode_bool(deserializer)"));
        assert!(output.contains("return (var_first, var_second);"));
    }
}
