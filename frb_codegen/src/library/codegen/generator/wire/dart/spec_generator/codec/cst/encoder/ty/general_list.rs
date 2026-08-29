use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};

impl WireDartCodecCstGeneratorEncoderTrait for GeneralListWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        let ident = self.mir.safe_ident();
        let inner = self.mir.inner.safe_ident();

        Acc {
            io: Some(format!(
                "final ans = wire.cst_new_{ident}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    {}
                }}
                return ans;
                ",
                if self.mir.inner.is_primitive()
                    || matches!(
                        *self.mir.inner,
                        MirType::Optional(_)
                            | MirType::RustAutoOpaqueImplicit(_)
                            | MirType::RustOpaque(_)
                            | MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(_))
                            | MirType::DartOpaque(_)
                            | MirType::PrimitiveList(_)
                            | MirType::Delegate(MirTypeDelegate::String)
                            | MirType::Delegate(MirTypeDelegate::StreamSink(_))
                            | MirType::Delegate(MirTypeDelegate::Time(_))
                            | MirType::Delegate(MirTypeDelegate::Uuid)
                            | MirType::Delegate(MirTypeDelegate::SerdeJsonValue)
                    )
                {
                    format!("ans.ref.ptr[i] = cst_encode_{inner}(raw[i]);")
                } else {
                    format!("cst_api_fill_to_wire_{inner}(raw[i], ans.ref.ptr[i]);")
                }
            )),
            web: self.context.config.web_enabled.then(|| {
                format!(
                    "return raw.map(cst_encode_{}).toList().jsify()!;",
                    self.mir.inner.safe_ident()
                )
            }),
            ..Default::default()
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => format!("ffi.Pointer<wire_cst_{}>", self.mir.safe_ident()),
            Target::Web => "JSAny".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::ty::general_list::MirTypeGeneralList;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::structure::{MirStructIdent, MirTypeStructRef};
    use crate::utils::namespace::{Namespace, NamespacedName};

    /// Uses direct and fill-to-wire element encoders and enables web list mapping.
    #[test]
    fn general_list_encoder_emits_io_allocation_and_web_mapping() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let generator = GeneralListWireDartCodecCstGenerator::new(
            MirTypeGeneralList {
                inner: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
            },
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );

        let output = generator.generate_encode_func_body();
        assert_eq!(
            output.io.as_deref(),
            Some(
                "final ans = wire.cst_new_list_i_32(raw.length);\n                for (var i = 0; i < raw.length; ++i) {\n                    ans.ref.ptr[i] = cst_encode_i_32(raw[i]);\n                }\n                return ans;\n                "
            )
        );
        assert_eq!(
            output.web.as_deref(),
            Some("return raw.map(cst_encode_i_32).toList().jsify()!;")
        );

        let struct_generator = GeneralListWireDartCodecCstGenerator::new(
            MirTypeGeneralList {
                inner: Box::new(MirType::StructRef(MirTypeStructRef {
                    ident: MirStructIdent(NamespacedName::new(
                        Namespace::default(),
                        "Point".into(),
                    )),
                    is_exception: false,
                })),
            },
            test_utils::context(
                &pack,
                &wire_dart_config,
                &wire_rust_config,
                &api_dart_config,
            ),
        );
        assert_eq!(
            struct_generator.generate_encode_func_body().io.as_deref(),
            Some(
                "final ans = wire.cst_new_list_point(raw.length);\n                for (var i = 0; i < raw.length; ++i) {\n                    cst_api_fill_to_wire_point(raw[i], ans.ref.ptr[i]);\n                }\n                return ans;\n                "
            )
        );
    }
}
