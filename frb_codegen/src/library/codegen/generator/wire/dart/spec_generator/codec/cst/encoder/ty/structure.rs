use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::lang::dart::DartLang;
use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::generator::misc::struct_or_record::StructOrRecord;
use crate::codegen::generator::misc::struct_or_record::StructOrRecord::Struct;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::misc::dart_wire_type_from_rust_wire_type_or_web;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::structure::{MirStruct, MirTypeStructRef};
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for StructRefWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        self.new_generalized_generator().generate_encode_func_body()
    }

    fn generate_encode_api_fill_to_wire_body(&self) -> Option<String> {
        self.new_generalized_generator().api_fill_to_wire_body()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        dart_wire_type_from_rust_wire_type_or_web(self, target, "JSAny".into())
    }
}

impl<'a> StructRefWireDartCodecCstGenerator<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.mir.clone(), self.context, Struct)
    }
}

pub(crate) struct GeneralizedStructGenerator<'a> {
    mir: MirTypeStructRef,
    context: WireDartCodecCstGeneratorContext<'a>,
    mode: StructOrRecord,
}

impl<'a> GeneralizedStructGenerator<'a> {
    pub(crate) fn new(
        mir: MirTypeStructRef,
        context: WireDartCodecCstGeneratorContext<'a>,
        mode: StructOrRecord,
    ) -> Self {
        Self { mir, context, mode }
    }

    pub(crate) fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        Acc {
            web: self.context.config.web_enabled.then(|| {
                let st = self.mir.get(self.context.mir_pack);
                let values = (st.fields.iter().enumerate())
                    .map(|(index, field)| {
                        format!(
                            "cst_encode_{}(raw.{})",
                            field.ty.safe_ident(),
                            self.mode.field_name(
                                index,
                                field,
                                st.is_fields_named,
                                &Lang::DartLang(DartLang)
                            )
                        )
                    })
                    .join(",");
                format!("return [{}].jsify()!;", values)
            }),
            ..Default::default()
        }
    }

    pub(crate) fn api_fill_to_wire_body(&self) -> Option<String> {
        let st = self.mir.get(self.context.mir_pack);
        Some(
            st.fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    self.generate_api_fill_to_wire_body_struct_field(index, field, st)
                })
                .collect_vec()
                .join("\n"),
        )
    }

    fn generate_api_fill_to_wire_body_struct_field(
        &self,
        index: usize,
        field: &MirField,
        st: &MirStruct,
    ) -> String {
        let safe_ident = field.ty.safe_ident();
        let dart_style =
            (self.mode).field_name(index, field, st.is_fields_named, &Lang::DartLang(DartLang));
        let c_style = field.name.c_style();

        if field.ty.is_struct_or_enum_or_record() {
            format!("cst_api_fill_to_wire_{safe_ident}(apiObj.{dart_style}, wireObj.{c_style});")
        } else {
            format!("wireObj.{c_style} = cst_encode_{safe_ident}(apiObj.{dart_style});")
        }
    }
}
