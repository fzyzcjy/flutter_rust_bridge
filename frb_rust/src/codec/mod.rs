pub(crate) trait BaseCodec {}

pub(crate) struct CstCodec;
impl BaseCodec for CstCodec {}

pub(crate) struct DcoCodec;
impl BaseCodec for DcoCodec {}

pub(crate) struct SseCodec;
impl BaseCodec for SseCodec {}
