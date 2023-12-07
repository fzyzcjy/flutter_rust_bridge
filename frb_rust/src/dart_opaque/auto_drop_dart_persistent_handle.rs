use dart_sys::{Dart_DeletePersistentHandle_DL, Dart_Handle, Dart_HandleFromPersistent_DL};
use dart_sys::{Dart_NewPersistentHandle_DL, Dart_PersistentHandle};

#[derive(Debug)]
/// A [Dart_PersistentHandle] that delete the handle when `Drop`ped
// `Option` is used for correct drop.
pub struct AutoDropDartPersistentHandle(Option<Dart_PersistentHandle>);

impl AutoDropDartPersistentHandle {
    pub fn new_from_non_persistent_handle(non_persistent_handle: Dart_Handle) -> Self {
        println!(
            "hi new_from_non_persistent_handle call Dart_NewPersistentHandle_DL non_persistent_handle={non_persistent_handle:?} thread={:?}",
            std::thread::current().id()
        );
        unsafe {
            let persistent_handle = Dart_NewPersistentHandle_DL
                .expect("dart_api_dl has not been initialized")(
                non_persistent_handle
            );
            println!("hi new_from_non_persistent_handle persistent_handle={persistent_handle:?}");
            let ans = Self::from_raw(persistent_handle);

            println!("hi call extra create_dart_handle");
            ans.create_dart_handle();

            ans
        }
    }

    // `from_raw` is `unsafe` while `into_raw` is not, mimicking `Box::*` counterpart.
    unsafe fn from_raw(ptr: Dart_PersistentHandle) -> Self {
        Self(Some(ptr))
    }

    /// https://github.com/dart-lang/sdk/blob/af20a8ab0394408ee48483c5c06c75281e7ba52c/runtime/include/dart_api.h#L424C8-L424C8
    /// "Allocates a handle in the current scope from a persistent handle."
    pub fn create_dart_handle(&self) -> Dart_Handle {
        println!(
            "hi call Dart_HandleFromPersistent_DL START self={self:?} thread={:?}",
            std::thread::current().id()
        );
        unsafe {
            let ans = Dart_HandleFromPersistent_DL.expect("dart_api_dl has not been initialized")(
                self.0.unwrap(),
            );
            println!("hi call Dart_HandleFromPersistent_DL END ans={ans:?}",);

            ans
        }
    }
}

impl Drop for AutoDropDartPersistentHandle {
    fn drop(&mut self) {
        if let Some(inner) = self.0 {
            unsafe {
                println!("hi call Dart_DeletePersistentHandle_DL");
                Dart_DeletePersistentHandle_DL.expect("dart_api_dl has not been initialized")(inner)
            }
        }
    }
}
