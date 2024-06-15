use crate::codegen::generator::codec::sse::lang::Lang;
use crate::codegen::ir::mir::field::MirField;

pub(crate) enum StructOrRecord {
    Struct,
    Record,
}

impl StructOrRecord {
    pub(crate) fn field_name(
        &self,
        index: usize,
        field: &MirField,
        is_field_named: bool,
        lang: &Lang,
    ) -> String {
        match lang {
            Lang::DartLang(_) => match self {
                StructOrRecord::Struct => field.name.dart_style(),
                StructOrRecord::Record => format!("${}", index + 1),
            },
            Lang::RustLang(_) => match self {
                StructOrRecord::Struct => {
                    if is_field_named {
                        field.name.rust_style().to_owned()
                    } else {
                        format!("{}", index)
                    }
                }
                StructOrRecord::Record => format!("{}", index),
            },
        }
    }
}
