use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::rust_wire_type_add_prefix_or_js_value;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for RecordWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        self.as_struct_generator().generate_decoder_class()
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        let mir = self.mir.inner.get(self.context.mir_pack);
        let len = mir.fields.len();
        let values: Acc<Vec<_>> = mir
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| Acc {
                web: format!("self_.get({idx}).cst_decode()"),
                io: format!("self.{}.cst_decode()", field.name.rust_style()),
                ..Default::default()
            })
            .collect();
        Acc {
            io: Some(format!("({},)", values.io.join(","))),
            web: Some(format!(
                "let self_ = self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                ({},)",
                values.web.join(",")
            )),
            ..Default::default()
        }
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        self.as_struct_generator().generate_impl_new_with_nullptr()
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.mir, target)
    }
}

impl RecordWireRustCodecCstGenerator<'_> {
    pub(crate) fn as_struct_generator(&self) -> StructRefWireRustCodecCstGenerator {
        StructRefWireRustCodecCstGenerator {
            mir: self.mir.inner.clone(),
            context: self.context,
        }
    }
}
