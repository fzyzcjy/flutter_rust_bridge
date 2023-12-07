#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransferMode {
    Cst,
    Dco,
    Sse,
}
