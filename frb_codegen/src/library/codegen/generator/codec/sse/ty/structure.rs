use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::misc::StructOrRecord;
use crate::codegen::generator::misc::StructOrRecord::Struct;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use itertools::Itertools;

impl<'a> CodecSseTyTrait for StructRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        self.new_generalized_generator().generate_encode(lang)
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        self.new_generalized_generator().generate_decode(lang)
    }
}

impl<'a> StructRefCodecSseTy<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.ir.clone(), self.context, Struct)
    }
}

pub(crate) struct GeneralizedStructGenerator<'a> {
    ir: IrTypeStructRef,
    context: CodecSseTyContext<'a>,
    mode: StructOrRecord,
}

impl<'a> GeneralizedStructGenerator<'a> {
    pub(crate) fn new(
        ir: IrTypeStructRef,
        context: CodecSseTyContext<'a>,
        mode: StructOrRecord,
    ) -> Self {
        Self { ir, context, mode }
    }

    pub(super) fn generate_encode(&self, lang: &Lang) -> String {
        let st = self.ir.get(self.context.ir_pack);
        (st.fields.iter().enumerate())
            .map(|(index, field)| {
                format!(
                    "{};\n",
                    lang.call_encode(
                        &field.ty,
                        &format!("src.{}", self.mode.field_name_dart_style(index, field))
                    )
                )
            })
            .join("")
    }

    pub(super) fn generate_decode(&self, lang: &Lang) -> String {
        let st = self.ir.get(self.context.ir_pack);
        let decode_fields = (st.fields.iter().enumerate())
            .map(|(index, field)| {
                format!(
                    "{} {} = {};\n",
                    lang.var_decl(),
                    self.mode.field_name_dart_style(index, field),
                    lang.call_decode(&field.ty)
                )
            })
            .join("");
        format!(
            "
            {decode_fields}
            return {};
            ",
            lang.call_constructor(
                &st.name.name,
                &st.fields.iter().map(|x| x.name.raw.clone()).collect_vec()
            )
        )
    }
}
