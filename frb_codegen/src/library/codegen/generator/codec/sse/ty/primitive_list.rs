use crate::codegen::generator::codec::sse::ty::general_list::{
    general_list_generate_decode, general_list_generate_encode, list_len_method, LEN_TYPE,
};
use crate::codegen::generator::codec::sse::ty::primitive::get_serializer_dart_postfix;
use crate::codegen::generator::codec::sse::ty::*;
use crate::library::codegen::generator::codec::sse::lang::LangTrait;

impl<'a> CodecSseTyTrait for PrimitiveListCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(match lang {
            Lang::DartLang(_) => {
                let type_converter = if self.ir.strict_dart_type {
                    "self".to_owned()
                } else {
                    format!(
                        "self is {prim}List ? self : {prim}List.fromList(self)",
                        prim = get_serializer_dart_postfix(&self.ir.primitive),
                    )
                };

                format!(
                    "{};
                    serializer.buffer.put{}List({type_converter});",
                    lang.call_encode(&LEN_TYPE, &format!("self.{}", list_len_method(lang))),
                    get_serializer_dart_postfix(&self.ir.primitive)
                )
            }
            Lang::RustLang(_) => {
                // TODO do not use naive loop
                self.ir.strict_dart_type.then(|| {
                    general_list_generate_encode(
                        lang,
                        &IrType::Primitive(self.ir.primitive.clone()),
                    )
                })
            }
        })
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let var_decl = lang.var_decl();
        Some(match lang {
            Lang::DartLang(_) => format!(
                "{var_decl} len_ = {};
                return deserializer.buffer.get{}List(len_);",
                lang.call_decode(&LEN_TYPE),
                get_serializer_dart_postfix(&self.ir.primitive)
            ),
            Lang::RustLang(_) => {
                // TODO do not use naive loop
                self.ir.strict_dart_type.then(|| {
                    general_list_generate_decode(
                        lang,
                        &IrType::Primitive(self.ir.primitive.clone()),
                        self.context,
                    )
                })
            }
        })
    }
}
