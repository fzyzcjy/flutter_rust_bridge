use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};
use itertools::Itertools;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for StructRefWireRustCodecCstGenerator<'a> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        let s = self.mir.get(self.context.mir_pack);
        Some(generate_class_from_fields(
            self.mir.clone(),
            self.context,
            &s.fields
                .iter()
                .map(|field| {
                    let field_generator =
                        WireRustCodecCstGenerator::new(field.ty.clone(), self.context);
                    format!(
                        "{}: {}{}",
                        field.name.rust_style(),
                        field_generator.rust_wire_modifier(Target::Io),
                        field_generator.rust_wire_type(Target::Io)
                    )
                })
                .collect_vec(),
        ))
    }

    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        let api_struct = self.mir.get(self.context.mir_pack);
        let fields: Acc<Vec<_>> = api_struct
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let field_name = field.name.rust_style();
                let field_ = if api_struct.is_fields_named {
                    format!("{field_name}: ")
                } else {
                    String::new()
                };

                Acc {
                    web: format!("{field_} self_.get({idx}).cst_decode()"),
                    io: format!("{field_} self.{field_name}.cst_decode()"),
                    ..Default::default()
                }
            })
            .collect();

        let (left, right) = api_struct.brackets_pair();
        let rust_api_type = self.mir.rust_api_type();
        Acc {
            io: Some(format!(
                "
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.io.join(","),
            )),
            web: Some(format!(
                "
                let self_ = self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.web.join(","),
                len = api_struct.fields.len(),
            )),
            ..Default::default()
        }
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        let src = self.mir.get(self.context.mir_pack);

        let body = {
            src.fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {},",
                        field.name.rust_style(),
                        if WireRustCodecCstGenerator::new(field.ty.clone(), self.context)
                            .rust_wire_is_pointer(Target::Io)
                            || matches!(field.ty, MirType::DartOpaque(_))
                        {
                            "core::ptr::null_mut()".to_owned()
                        } else {
                            "Default::default()".to_owned()
                        }
                    )
                })
                .collect_vec()
                .join("\n")
        };

        Some(
            generate_impl_new_with_nullptr_code_block(
                self.mir.clone(),
                self.context,
                &format!("Self {{ {body} }}"),
            )
            .into(),
        )
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.mir, target)
    }
}
