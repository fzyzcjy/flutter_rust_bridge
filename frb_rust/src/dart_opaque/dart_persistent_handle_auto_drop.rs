use dart_sys::{Dart_DeletePersistentHandle_DL, Dart_Handle, Dart_HandleFromPersistent_DL};

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

    /// https://github.com/dart-lang/sdk/blob/af20a8ab0394408ee48483c5c06c75281e7ba52c/runtime/include/dart_api.h#L424C8-L424C8
    /// "Allocates a handle in the current scope from a persistent handle."
    pub fn create_dart_handle(&self) -> Dart_Handle {
        unsafe {
            Dart_HandleFromPersistent_DL.expect("dart_api_dl has not been initialized")(
                self.0.unwrap(),
            );
        }
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
