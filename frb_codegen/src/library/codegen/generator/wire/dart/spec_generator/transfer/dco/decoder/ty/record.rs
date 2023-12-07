use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait for RecordWireDartTransferDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        let len = self.ir.values.len();
        let values = self
            .ir
            .values
            .iter()
            .enumerate()
            .map(|(idx, ty)| format!("_dco_decode_{}(arr[{idx}])", ty.safe_ident()))
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
