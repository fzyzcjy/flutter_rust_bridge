use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::enumeration::{
    generate_enum_access_object_core, parse_wrapper_name_into_dart_name_and_self_path,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegatePrimitiveEnum};
use itertools::Itertools;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for DelegateWireRustCodecDcoGenerator<'a> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn generate_impl_into_dart(&self) -> Option<String> {
        // frb-coverage:ignore-end
        if let MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum { mir, .. }) = &self.mir
        {
            let src = mir.get(self.context.mir_pack);
            let (name, self_path) =
                parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

            let self_ref = generate_enum_access_object_core(mir, "self".to_owned(), self.context);
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    format!("{}::{} => {}.into_dart(),", self_path, variant.name, idx)
                })
                .collect_vec()
                .join("\n");

            let body = format!(
                "match {self_ref} {{
                    {variants}
                    _ => unreachable!(),
                }}"
            );

            return Some(
                generate_impl_into_dart(&name, &body)
                    + &generate_impl_into_into_dart(&src.name.rust_style(), &src.wrapper_name),
            );
        }
        None
    }
}
