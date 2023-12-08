use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for PrimitiveCodecSseTy<'a> {
    fn generate_encode(&self, _lang: &impl Lang) -> String {
        todo!()
    }

    fn generate_decode(&self, _lang: &impl Lang) -> String {
        todo!()
    }
}
