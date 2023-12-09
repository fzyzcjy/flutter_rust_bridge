use crate::codegen::generator::codec::sse::ty::general_list::general_list_generate_decode;
use crate::codegen::generator::codec::sse::ty::primitive::get_serializer_dart_postfix;
use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveListCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        Some(match lang {
            Lang::DartLang(_) => format!(
                "serializer.buffer.put{}List(self);",
                get_serializer_dart_postfix(&self.ir.primitive)
            ),
            Lang::RustLang(_) => format!(
                "for item in self {{ serializer.cursor.write_{}::<NativeEndian>(item).unwrap(); }}",
                self.ir.primitive.rust_api_type()
            ),
        })
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        Some(match lang {
            Lang::DartLang(_) => format!(
                "return deserializer.buffer.get{}List();",
                get_serializer_dart_postfix(&self.ir.primitive)
            ),
            Lang::RustLang(_) => general_list_generate_decode(lang, &self.ir.primitive),
        })
    }
}
