use crate::codegen::generator::codec::sse::ty::*;

impl CodecSseTyTrait for GenericCodecSseTy<'_> {
    fn generate_encode(&self, _lang: &Lang) -> Option<String> {
        // Generic types should not appear in generated code - they are placeholders
        None
    }

    fn generate_decode(&self, _lang: &Lang) -> Option<String> {
        // Generic types should not appear in generated code - they are placeholders
        None
    }
}

