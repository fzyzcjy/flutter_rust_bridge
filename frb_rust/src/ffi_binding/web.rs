/// # Safety
///
/// TODO: need doc
#[wasm_bindgen]
pub unsafe fn get_dart_object(ptr: usize) -> JsValue {
    *support::box_from_leak_ptr(ptr as _)
}

/// # Safety
///
/// TODO: need doc
#[wasm_bindgen]
pub unsafe fn drop_dart_object(ptr: usize) {
    drop(support::box_from_leak_ptr::<JsValue>(ptr as _));
}

