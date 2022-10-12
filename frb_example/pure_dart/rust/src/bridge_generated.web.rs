use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test42(port_: MessagePort) {
    wire_test42_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_opaque(port_: MessagePort, value: JsValue) {
    wire_handle_opaque_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_handle_opaque_repr(port_: MessagePort, value: *mut wire_RwLockI32) {
    wire_handle_opaque_repr_impl(port_, value)
}

// Section: allocate functions

// Section: impl Wire2Api

impl Wire2Api<OpaqueBag> for JsValue {
    fn wire2api(self) -> OpaqueBag {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        OpaqueBag {
            primitive: self_.get(0).wire2api(),
            array: self_.get(1).wire2api(),
            lifetime: self_.get(2).wire2api(),
            trait_obj: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<Option<OpaqueBag>> for JsValue {
    fn wire2api(self) -> Option<OpaqueBag> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
// Section: impl Wire2Api for JsValue
