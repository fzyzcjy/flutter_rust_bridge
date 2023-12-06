#[derive(Debug)]
/// A [Dart_PersistentHandle] that delete the handle when `Drop`ped
// `Option` is used for correct drop.
pub struct DartPersistentHandleAutoDrop(Option<Dart_PersistentHandle>);

impl DartPersistentHandleAutoDrop {
    // `from_raw` is `unsafe` while `into_raw` is not, mimicking `Box::*` counterpart.
    pub unsafe fn from_raw(ptr: Dart_PersistentHandle) -> Self {
        Self(Some(ptr))
    }

    pub fn into_raw(mut self) -> Dart_PersistentHandle {
        self.0.take().unwrap()
    }
}

impl Drop for DartPersistentHandleAutoDrop {
    fn drop(&mut self) {
        if let Some(inner) = self.0 {
            unsafe {
                Dart_DeletePersistentHandle_DL.expect("dart_api_dl has not been initialized")(inner)
            }
        }
    }
}
