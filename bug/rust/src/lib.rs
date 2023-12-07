use dart_sys::*;

#[no_mangle]
pub unsafe extern "C" fn naive_NewPersistentHandle(non_persistent_handle: Dart_Handle) -> usize {
    println!("hi naive_NewPersistentHandle START non_persistent_handle={non_persistent_handle:?}");
    let persistent_handle = Dart_NewPersistentHandle_DL.unwrap()(non_persistent_handle);
    println!("hi naive_NewPersistentHandle END non_persistent_handle={non_persistent_handle:?} persistent_handle={persistent_handle:?}");
    persistent_handle as _
}

#[no_mangle]
pub unsafe extern "C" fn naive_HandleFromPersistent(persistent_handle: usize) -> usize {
    println!("hi naive_HandleFromPersistent START persistent_handle={persistent_handle:?}");
    let ans = Dart_HandleFromPersistent_DL.unwrap()(persistent_handle as _);
    println!(
        "hi naive_HandleFromPersistent END persistent_handle={persistent_handle:?} ans={ans:?}"
    );
    ans as _
}
