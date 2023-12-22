use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for OwnershipCodecSseTy<'a> {
    // frb-coverage:ignore-start
    fn generate_encode(&self, _lang: &Lang) -> Option<String> {
        unreachable!()
    }

    fn generate_decode(&self, _lang: &Lang) -> Option<String> {
        unreachable!()
    }
    // frb-coverage:ignore-end
}
