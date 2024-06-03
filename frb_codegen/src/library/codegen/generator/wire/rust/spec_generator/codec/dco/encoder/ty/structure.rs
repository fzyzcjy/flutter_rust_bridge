use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::enumeration::parse_wrapper_name_into_dart_name_and_self_path;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use itertools::Itertools;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for StructRefWireRustCodecDcoGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);

        let unwrap = match &src.wrapper_name {
            Some(_) => ".0",
            None => "",
        };
        let body = src
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let field_ref = if src.is_fields_named {
                    field.name.rust_style().to_string()
                } else {
                    i.to_string()
                };

                format!("self{unwrap}.{field_ref}.into_into_dart().into_dart()")
            })
            .collect_vec()
            .join(",\n");

        let (name, _) =
            parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

        let body = if src.is_empty() {
            "Vec::<u8>::new().into_dart()".to_string()
        } else {
            format!(
                "[
                    {body}
                ].into_dart()"
            )
        };

        Some(
            generate_impl_into_dart(&name, &body)
                + &generate_impl_into_into_dart(&src.name.rust_style(), &src.wrapper_name),
        )
    }
}
