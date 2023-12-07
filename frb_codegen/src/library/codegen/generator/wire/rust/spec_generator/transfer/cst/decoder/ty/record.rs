use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::misc::rust_wire_type_add_prefix_or_js_value;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;

impl<'a> WireRustTransferCstGeneratorDecoderTrait for RecordWireRustTransferCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<String> {
        self.as_struct_generator().generate_decoder_class()
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        let ir = self.ir.inner.get(self.context.ir_pack);
        let len = ir.fields.len();
        let values: Acc<Vec<_>> = ir
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| Acc {
                wasm: format!("self_.get({idx}).cst_decode()"),
                io: format!("self.{}.cst_decode()", field.name.rust_style()),
                ..Default::default()
            })
            .collect();
        Acc {
            io: Some(format!("({},)", values.io.join(","))),
            wasm: Some(format!(
                "let self_ = self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                ({},)",
                values.wasm.join(",")
            )),
            ..Default::default()
        }
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        self.as_struct_generator().generate_impl_new_with_nullptr()
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl RecordWireRustTransferCstGenerator<'_> {
    pub(crate) fn as_struct_generator(&self) -> StructRefWireRustTransferCstGenerator {
        StructRefWireRustTransferCstGenerator {
            ir: self.ir.inner.clone(),
            context: self.context,
        }
    }
}
