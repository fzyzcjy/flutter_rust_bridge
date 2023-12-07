pub(crate) trait TransferHandler {}

pub(crate) struct CstTransferHandler;
impl TransferHandler for CstTransferHandler {}

pub(crate) struct DcoTransferHandler;
impl TransferHandler for DcoTransferHandler {}

pub(crate) struct SseTransferHandler;
impl TransferHandler for SseTransferHandler {}
