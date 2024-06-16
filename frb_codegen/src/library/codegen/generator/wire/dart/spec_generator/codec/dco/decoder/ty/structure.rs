use crate::codegen::generator::api_dart::spec_generator::class::method::dart_constructor_postfix;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for StructRefWireDartCodecDcoGenerator<'a> {
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
