use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for GeneralListWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        if let MirType::Delegate(MirTypeDelegate::Uuid) = &*self.mir.inner {
            return "const kUuidSizeInBytes = 16;
                final bytes = dco_decode_list_prim_u_8_strict(raw);
                return List.generate(
                  bytes.lengthInBytes ~/ kUuidSizeInBytes,
                  (i) => UuidValue.fromByteList(Uint8List.view(bytes.buffer, i * kUuidSizeInBytes, kUuidSizeInBytes)),
                  growable: false,
                );".to_owned();
        }

        format!(
            "return dcoDecodeList(raw).map(dco_decode_{}).toList();",
            self.mir.inner.safe_ident()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::general_list::MirTypeGeneralList;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

    /// Decodes UUID lists by fixed-size byte views and ordinary lists by mapping inner decoders.
    #[test]
    fn general_list_decoder_covers_uuid_and_generic_list_branches() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let context = test_utils::context(&pack, &config);
        let uuid = GeneralListWireDartCodecDcoGenerator::new(
            MirTypeGeneralList {
                inner: Box::new(MirType::Delegate(MirTypeDelegate::Uuid)),
            },
            context,
        );
        assert_eq!(
            uuid.generate_impl_decode_body(),
            "const kUuidSizeInBytes = 16;\n                final bytes = dco_decode_list_prim_u_8_strict(raw);\n                return List.generate(\n                  bytes.lengthInBytes ~/ kUuidSizeInBytes,\n                  (i) => UuidValue.fromByteList(Uint8List.view(bytes.buffer, i * kUuidSizeInBytes, kUuidSizeInBytes)),\n                  growable: false,\n                );"
        );

        let generic = GeneralListWireDartCodecDcoGenerator::new(
            MirTypeGeneralList {
                inner: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
            },
            context,
        );
        assert_eq!(
            generic.generate_impl_decode_body(),
            "return dcoDecodeList(raw).map(dco_decode_i_32).toList();"
        );
    }
}
