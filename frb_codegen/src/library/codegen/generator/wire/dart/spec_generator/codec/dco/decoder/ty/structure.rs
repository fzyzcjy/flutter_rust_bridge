use crate::codegen::generator::api_dart::spec_generator::class::method::dart_constructor_postfix;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl WireDartCodecDcoGeneratorDecoderTrait for StructRefWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        let s = self.mir.get(self.context.mir_pack);

        let inner = s
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                format!(
                    "{}: dco_decode_{}(arr[{}]),",
                    field.name.dart_style(),
                    field.ty.safe_ident(),
                    idx
                )
            })
            .collect_vec();

        let inner = inner.join("\n");
        let cast = "final arr = raw as List<dynamic>;".to_string();
        let safe_check = format!("if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');", s.fields.len(), s.fields.len());
        let ctor_postfix = dart_constructor_postfix(
            &s.name.name,
            &self.context.mir_pack.funcs_with_impl(),
            self.context.as_api_dart_context(),
        );
        format!(
            "{cast}
                {safe_check}
                return {name}{ctor_postfix}({inner});",
            name = s.name.name,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent, MirTypeStructRef};
    use crate::codegen::ir::mir::ty::MirType;
    use crate::utils::namespace::{Namespace, NamespacedName};

    fn field(name: &str, ty: MirType) -> MirField {
        MirField {
            ty,
            name: MirIdent::new(name.into(), None),
            is_final: true,
            is_rust_public: None,
            comments: vec![],
            default: None,
            settings: MirFieldSettings::default(),
        }
    }

    /// Emits named constructor arguments and an exact structure arity guard.
    #[test]
    fn structure_decoder_checks_arity_and_uses_dart_field_names() {
        let mut pack = test_utils::pack();
        let name = NamespacedName::new(Namespace::default(), "Point".into());
        pack.struct_pool.insert(
            MirStructIdent(name.clone()),
            MirStruct {
                name: name.clone(),
                wrapper_name: None,
                fields: vec![field("x_value", MirType::Primitive(MirTypePrimitive::I32))],
                is_fields_named: true,
                dart_metadata_raw: vec![],
                ignore: false,
                needs_json_serializable: false,
                generate_hash: false,
                generate_eq: false,
                dart_collection_deep_equality: false,
                ui_state: false,
                comments: vec![],
            },
        );
        let config = test_utils::config();
        let generator = StructRefWireDartCodecDcoGenerator::new(
            MirTypeStructRef {
                ident: MirStructIdent(name),
                is_exception: false,
            },
            test_utils::context(&pack, &config),
        );
        let output = generator.generate_impl_decode_body();

        assert_eq!(
            output,
            "final arr = raw as List<dynamic>;\n                if (arr.length != 1) throw Exception('unexpected arr length: expect 1 but see ${arr.length}');\n                return Point(xValue: dco_decode_i_32(arr[0]),);"
        );
    }
}
