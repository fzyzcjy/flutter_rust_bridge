use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for TraitDefWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        "throw UnimplementedError();".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
    use crate::utils::namespace::{Namespace, NamespacedName};

    /// Rejects DCO decoding for trait definitions.
    #[test]
    fn trait_definition_decoder_remains_unimplemented() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let generator = TraitDefWireDartCodecDcoGenerator::new(
            MirTypeTraitDef {
                name: NamespacedName::new(Namespace::default(), "Service".into()),
            },
            test_utils::context(&pack, &config),
        );

        assert_eq!(
            generator.generate_impl_decode_body(),
            "throw UnimplementedError();"
        );
    }
}
