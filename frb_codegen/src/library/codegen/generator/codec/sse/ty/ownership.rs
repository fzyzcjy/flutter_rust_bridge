use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for OwnershipCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> String {
        unreachable!()
    }

    fn generate_decode(&self, lang: &Lang) -> String {
        unreachable!()
    }
}
