use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_enum_method__method__Weekdays(port_: MessagePort, that: i32, message: String) {
    wire_test_enum_method__method__Weekdays_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_enum_method__static_method__Weekdays(port_: MessagePort, message: String) {
    wire_test_static_enum_method__static_method__Weekdays_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__CrossSharedStructInBlock1And2(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__CrossSharedStructInBlock1And2_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__CrossSharedStructInBlock1And2(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__CrossSharedStructInBlock1And2_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__SharedStructInAllBlocks(
    port_: MessagePort,
    that: JsValue,
    message: String,
    num: u32,
) {
    wire_test_method__method__SharedStructInAllBlocks_impl(port_, that, message, num)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__SharedStructInAllBlocks(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__SharedStructInAllBlocks_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__SharedStructInBlock1And2(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__SharedStructInBlock1And2_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__SharedStructInBlock1And2(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__SharedStructInBlock1And2_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__SharedStructOnlyForSyncTest(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__SharedStructOnlyForSyncTest_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__SharedStructOnlyForSyncTest(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__SharedStructOnlyForSyncTest_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__SharedStructInBlock2And3(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__SharedStructInBlock2And3_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__SharedStructInBlock2And3(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__SharedStructInBlock2And3_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__CrossSharedStructInBlock2And3(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__CrossSharedStructInBlock2And3_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__CrossSharedStructInBlock2And3(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__CrossSharedStructInBlock2And3_impl(port_, message)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<ZeroCopyBuffer<Vec<f32>>> for Box<[f32]> {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<f32>> {
        ZeroCopyBuffer(self.wire2api())
    }
}

impl Wire2Api<CrossSharedStructInBlock1And2> for JsValue {
    fn wire2api(self) -> CrossSharedStructInBlock1And2 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        CrossSharedStructInBlock1And2 {
            name: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<CrossSharedStructInBlock2And3> for JsValue {
    fn wire2api(self) -> CrossSharedStructInBlock2And3 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        CrossSharedStructInBlock2And3 {
            name: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<EnumType> for JsValue {
    fn wire2api(self) -> EnumType {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumType::Empty,
            1 => EnumType::Primitives {
                int32: self_.get(1).wire2api(),
                float64: self_.get(2).wire2api(),
                boolean: self_.get(3).wire2api(),
            },
            2 => EnumType::Nested(self_.get(1).wire2api()),
            3 => EnumType::Optional(self_.get(1).wire2api(), self_.get(2).wire2api()),
            4 => EnumType::Buffer(self_.get(1).wire2api()),
            5 => EnumType::Enums(self_.get(1).wire2api()),
            6 => EnumType::BytesArray(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<f32>> for Box<[f32]> {
    fn wire2api(self) -> Vec<f32> {
        self.into_vec()
    }
}

impl Wire2Api<Vec<EnumType>> for JsValue {
    fn wire2api(self) -> Vec<EnumType> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}

impl Wire2Api<Option<Vec<EnumType>>> for JsValue {
    fn wire2api(self) -> Option<Vec<EnumType>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<u8>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<Vec<u8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<SharedStructInAllBlocks> for JsValue {
    fn wire2api(self) -> SharedStructInAllBlocks {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        SharedStructInAllBlocks {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
            enum_list: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<SharedStructInBlock1And2> for JsValue {
    fn wire2api(self) -> SharedStructInBlock1And2 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        SharedStructInBlock1And2 {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
impl Wire2Api<SharedStructInBlock2And3> for JsValue {
    fn wire2api(self) -> SharedStructInBlock2And3 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        SharedStructInBlock2And3 {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
impl Wire2Api<SharedStructOnlyForSyncTest> for JsValue {
    fn wire2api(self) -> SharedStructOnlyForSyncTest {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        SharedStructOnlyForSyncTest {
            name: self_.get(0).wire2api(),
            score: self_.get(1).wire2api(),
        }
    }
}

impl Wire2Api<[u8; 3]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 3] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
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
impl Wire2Api<ZeroCopyBuffer<Vec<f32>>> for JsValue {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<f32>> {
        ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<Box<EnumType>> for JsValue {
    fn wire2api(self) -> Box<EnumType> {
        Box::new(self.wire2api())
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
impl Wire2Api<Vec<f32>> for JsValue {
    fn wire2api(self) -> Vec<f32> {
        self.unchecked_into::<js_sys::Float32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Option<i32>> for JsValue {
    fn wire2api(self) -> Option<i32> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<Option<Vec<u8>>> for JsValue {
    fn wire2api(self) -> Option<Vec<u8>> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
    }
}
impl Wire2Api<u16> for JsValue {
    fn wire2api(self) -> u16 {
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
impl Wire2Api<[u8; 3]> for JsValue {
    fn wire2api(self) -> [u8; 3] {
        let vec: Vec<u8> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<u8>> for JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<js_sys::Uint8Array>().to_vec().into()
    }
}
impl Wire2Api<Weekdays> for JsValue {
    fn wire2api(self) -> Weekdays {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
