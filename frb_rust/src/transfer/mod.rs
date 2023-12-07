use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash)]
pub(crate) enum TransferMode {
    Cst,
    Dco,
    Sse,
}
