pub(crate) trait BaseCodec {}

pub struct CstCodec;
impl BaseCodec for CstCodec {}

pub struct DcoCodec;
impl BaseCodec for DcoCodec {}

pub struct SseCodec;
impl BaseCodec for SseCodec {}
