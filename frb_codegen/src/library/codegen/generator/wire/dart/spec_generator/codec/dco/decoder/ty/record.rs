use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for RecordWireDartCodecDcoGenerator<'a> {
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
            "final arr = raw as List<dynamic>;
            if (arr.length != {len}) {{
                throw Exception('Expected {len} elements, got ${{arr.length}}');
            }}
            return ({values},);"
        )
    }
}
