use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl WireDartCodecDcoGeneratorDecoderTrait for RecordWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        let len = self.mir.values.len();
        let values = self
            .mir
            .values
            .iter()
            .enumerate()
            .map(|(idx, ty)| format!("dco_decode_{}(arr[{idx}])", ty.safe_ident()))
            .collect_vec()
            .join(",");
        format!(
            "final arr = dcoDecodeList(raw);
            if (arr.length != {len}) {{
                throw Exception('Expected {len} elements, got ${{arr.length}}');
            }}
            return ({values},);"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::structure::{MirStructIdent, MirTypeStructRef};
    use crate::codegen::ir::mir::ty::MirType;
    use crate::utils::namespace::{Namespace, NamespacedName};

    /// Emits an arity check and positional decoders for every record member.
    #[test]
    fn record_decoder_checks_arity_and_decodes_each_position() {
        let pack = test_utils::pack();
        let config = test_utils::config();
        let generator = RecordWireDartCodecDcoGenerator::new(
            crate::codegen::ir::mir::ty::record::MirTypeRecord {
                inner: MirTypeStructRef {
                    ident: MirStructIdent(NamespacedName::new(Namespace::default(), "Pair".into())),
                    is_exception: false,
                },
                values: vec![
                    MirType::Primitive(MirTypePrimitive::I32),
                    MirType::Primitive(MirTypePrimitive::Bool),
                ]
                .into_boxed_slice(),
            },
            test_utils::context(&pack, &config),
        );
        let output = generator.generate_impl_decode_body();

        assert_eq!(
            output,
            "final arr = dcoDecodeList(raw);\n            if (arr.length != 2) {\n                throw Exception('Expected 2 elements, got ${arr.length}');\n            }\n            return (dco_decode_i_32(arr[0]),dco_decode_bool(arr[1]),);"
        );
    }
}
