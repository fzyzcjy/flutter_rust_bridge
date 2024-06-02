use crate::codegen::generator::codec::sse::lang::rust::RustLang;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::codec::sse::ty::enumeration::generate_enum_encode_rust_general;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::enumeration::MirTypeEnumRef;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for EnumRefWireRustCodecDcoGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);
        let (name, _self_path) =
            parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);
        let self_ref = generate_enum_access_object_core(&self.mir, "self".to_owned(), self.context);

        let body = generate_enum_encode_rust_general(
            &Lang::RustLang(RustLang),
            src,
            &self_ref,
            |idx, variant| {
                let tag = format!("{idx}.into_dart()");
                let fields = (Some(tag).into_iter())
                    .chain(variant.kind.fields().iter().map(|field| {
                        format!("{}.into_into_dart().into_dart()", field.name.rust_style())
                    }))
                    .join(",\n");
                format!("[{fields}].into_dart()")
            },
        );

        Some(
            generate_impl_into_dart(&name, &body)
                + &generate_impl_into_into_dart(&src.name.rust_style(), &src.wrapper_name),
        )
    }
}

pub(super) fn generate_enum_access_object_core(
    mir: &MirTypeEnumRef,
    obj: String,
    context: WireRustCodecDcoGeneratorContext,
) -> String {
    let src = mir.get(context.mir_pack);
    match &src.wrapper_name {
        Some(_) => format!("{obj}.0"),
        None => obj,
    }
}

pub(super) fn parse_wrapper_name_into_dart_name_and_self_path(
    name: &NamespacedName,
    wrapper_name: &Option<String>,
) -> (String, String) {
    match &wrapper_name {
        Some(wrapper) => (wrapper.clone(), name.rust_style()),
        None => (name.rust_style(), "Self".into()),
    }
}
