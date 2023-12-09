use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::generator::misc::StructOrRecord;
use crate::codegen::generator::misc::StructOrRecord::Struct;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;
use itertools::Itertools;

impl<'a> CodecSseTyTrait for StructRefCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        self.new_generalized_generator().generate_encode(lang)
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        self.new_generalized_generator().generate_decode(lang, "")
    }
}

impl<'a> StructRefCodecSseTy<'a> {
    fn new_generalized_generator(&self) -> GeneralizedStructGenerator {
        GeneralizedStructGenerator::new(self.ir.get(self.context.ir_pack).clone(), Struct)
    }
}

pub(crate) struct GeneralizedStructGenerator {
    st: IrStruct,
    mode: StructOrRecord,
}

impl GeneralizedStructGenerator {
    pub(crate) fn new(st: IrStruct, mode: StructOrRecord) -> Self {
        Self { st, mode }
    }

    pub(super) fn generate_encode(&self, lang: &Lang) -> String {
        (self.st.fields.iter().enumerate())
            .map(|(index, field)| {
                format!(
                    "{};\n",
                    lang.call_encode(
                        &field.ty,
                        &format!("self.{}", self.mode.field_name(index, field, lang))
                    )
                )
            })
            .join("")
    }

    pub(super) fn generate_decode(&self, lang: &Lang, name_prefix: &str) -> String {
        let decode_fields = (self.st.fields.iter().enumerate())
            .map(|(index, field)| {
                format!(
                    "{} {} = {};\n",
                    lang.var_decl(),
                    field.name.dart_style(),
                    lang.call_decode(&field.ty)
                )
            })
            .join("");

        let ctor = match self.mode {
            Struct => lang.call_constructor(
                format!("{name_prefix}{}", self.st.name.name),
                &self
                    .st
                    .fields
                    .iter()
                    .map(|x| x.name.style(lang))
                    .collect_vec(),
                &(self.st.fields.iter())
                    .map(|x| x.name.dart_style().clone())
                    .collect_vec(),
            ),
            StructOrRecord::Record => format!(
                "({})",
                (self.st.fields.iter())
                    .map(|x| x.name.dart_style().clone())
                    .join(", ")
            ),
        };

        format!("{decode_fields}return {ctor};")
    }
}
