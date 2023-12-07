pub(crate) trait CodecHandler {}

pub(crate) struct CstCodecHandler;
impl CodecHandler for CstCodecHandler {}

pub(crate) struct DcoCodecHandler;
impl CodecHandler for DcoCodecHandler {}

pub(crate) struct SseCodecHandler;
impl CodecHandler for SseCodecHandler {}
