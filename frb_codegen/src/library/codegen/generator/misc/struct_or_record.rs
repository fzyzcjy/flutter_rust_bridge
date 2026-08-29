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
                        field.name.rust_style(false).to_owned()
                    } else {
                        format!("{index}")
                    }
                }
                StructOrRecord::Record => format!("{index}"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::sse::lang::{dart::DartLang, rust::RustLang};
    use crate::codegen::ir::mir::field::MirFieldSettings;
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

    fn field() -> MirField {
        MirField {
            ty: MirType::Primitive(MirTypePrimitive::I32),
            name: MirIdent::new("r#field_name".to_owned(), Some("dartName".to_owned())),
            is_final: false,
            is_rust_public: None,
            comments: vec![],
            default: None,
            settings: MirFieldSettings::default(),
        }
    }

    /// Formats struct and record fields for Dart's named and positional syntax.
    #[test]
    fn formats_dart_struct_and_record_field_names() {
        let field = field();

        assert_eq!(
            StructOrRecord::Struct.field_name(2, &field, true, &Lang::DartLang(DartLang)),
            "dartName"
        );
        assert_eq!(
            StructOrRecord::Record.field_name(2, &field, true, &Lang::DartLang(DartLang)),
            "$3"
        );
    }

    /// Formats Rust named and unnamed struct fields plus record fields by index.
    #[test]
    fn formats_rust_struct_and_record_field_names() {
        let field = field();

        assert_eq!(
            StructOrRecord::Struct.field_name(2, &field, true, &Lang::RustLang(RustLang)),
            "r#field_name"
        );
        assert_eq!(
            StructOrRecord::Struct.field_name(2, &field, false, &Lang::RustLang(RustLang)),
            "2"
        );
        assert_eq!(
            StructOrRecord::Record.field_name(2, &field, true, &Lang::RustLang(RustLang)),
            "2"
        );
    }
}
