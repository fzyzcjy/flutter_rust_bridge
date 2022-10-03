use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_rust_metrics(port_: MessagePort) {
    wire_rust_metrics_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_uuid(port_: MessagePort, id: Box<[u8]>) {
    wire_handle_uuid_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_handle_uuids(port_: MessagePort, ids: Box<[u8]>) {
    wire_handle_uuids_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_uuids_convert_to_strings(port_: MessagePort, ids: Box<[u8]>) {
    wire_handle_uuids_convert_to_strings_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_nested_uuids(port_: MessagePort, ids: JsValue) {
    wire_handle_nested_uuids_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_strings(port_: MessagePort, strings: JsValue) {
    wire_handle_strings_impl(port_, strings)
}

#[wasm_bindgen]
pub fn wire_send_i64(port_: MessagePort, value: i64) {
    wire_send_i64_impl(port_, value)
}

// Section: allocate functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<String>> for JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<uuid::Uuid> for Box<[u8]> {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for Box<[u8]> {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        wire2api_uuids(multiple)
    }
}

impl Wire2Api<FeatureUuid> for JsValue {
    fn wire2api(self) -> FeatureUuid {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        FeatureUuid {
            one: self_.get(0).wire2api(),
            many: self_.get(1).wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<String> for JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<uuid::Uuid> for JsValue {
    fn wire2api(self) -> uuid::Uuid {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for JsValue {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<u8> for JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
