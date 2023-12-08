use crate::codegen::generator::codec::sse::ty::*;

impl<'a> CodecSseTyTrait for UnencodableCodecSseTy<'a> {
    fn generate_encode(&self) -> String {
        unreachable!()
    }

    fn generate_decode(&self) -> String {
        unreachable!()
    }
}
