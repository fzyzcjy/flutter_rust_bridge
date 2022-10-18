use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_handle_uuids(port_: MessagePort, ids: Box<[u8]>) {
    wire_handle_uuids_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_uuids_convert_to_strings(port_: MessagePort, ids: Box<[u8]>) {
    wire_handle_uuids_convert_to_strings_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_strings(port_: MessagePort, strings: JsValue) {
    wire_handle_strings_impl(port_, strings)
}

#[wasm_bindgen]
pub fn wire_handle_bool(port_: MessagePort, input: bool) {
    wire_handle_bool_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_u32(port_: MessagePort, input: u32) {
    wire_handle_u32_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_u64(port_: MessagePort, input: u64) {
    wire_handle_u64_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_i8(port_: MessagePort, input: i8) {
    wire_handle_i8_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_i16(port_: MessagePort, input: i16) {
    wire_handle_i16_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_i32(port_: MessagePort, input: i32) {
    wire_handle_i32_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_i64(port_: MessagePort, input: i64) {
    wire_handle_i64_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_f32(port_: MessagePort, input: f32) {
    wire_handle_f32_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_f64(port_: MessagePort, input: f64) {
    wire_handle_f64_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_string(port_: MessagePort, input: String) {
    wire_handle_string_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_bool(input: bool) -> support::WireSyncReturnStruct {
    wire_handle_sync_bool_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_u32(input: u32) -> support::WireSyncReturnStruct {
    wire_handle_sync_u32_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_u64(input: u64) -> support::WireSyncReturnStruct {
    wire_handle_sync_u64_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i8(input: i8) -> support::WireSyncReturnStruct {
    wire_handle_sync_i8_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i16(input: i16) -> support::WireSyncReturnStruct {
    wire_handle_sync_i16_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i32(input: i32) -> support::WireSyncReturnStruct {
    wire_handle_sync_i32_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_i64(input: i64) -> support::WireSyncReturnStruct {
    wire_handle_sync_i64_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_f32(input: f32) -> support::WireSyncReturnStruct {
    wire_handle_sync_f32_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_f64(input: f64) -> support::WireSyncReturnStruct {
    wire_handle_sync_f64_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_sync_string(input: String) -> support::WireSyncReturnStruct {
    wire_handle_sync_string_impl(input)
}

#[wasm_bindgen]
pub fn wire_dummy(port_: MessagePort, unit: i32) {
    wire_dummy_impl(port_, unit)
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
impl Wire2Api<Vec<uuid::Uuid>> for Box<[u8]> {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        wire2api_uuids(multiple)
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
impl Wire2Api<Vec<uuid::Uuid>> for JsValue {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        self.unchecked_into::<js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<f32> for JsValue {
    fn wire2api(self) -> f32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i16> for JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i64> for JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
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
impl Wire2Api<Unit> for JsValue {
    fn wire2api(self) -> Unit {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
