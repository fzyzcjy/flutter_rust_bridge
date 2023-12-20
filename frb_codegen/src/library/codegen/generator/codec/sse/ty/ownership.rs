use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for OwnershipCodecSseTy<'a> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn generate_encode(&self, _lang: &Lang) -> Option<String> {
        unreachable!()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn generate_decode(&self, _lang: &Lang) -> Option<String> {
        unreachable!()
    }
}
