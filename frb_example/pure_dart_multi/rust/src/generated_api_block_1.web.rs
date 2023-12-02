use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_test_all_shared_struct_in_block_1(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_all_shared_struct_in_block_1_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_cross_shared_struct_in_block_1_for_1_and_2(port_: MessagePort, custom: JsValue) {
    wire_test_cross_shared_struct_in_block_1_for_1_and_2_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_enum_defined_in_block_1(port_: MessagePort, custom: JsValue) {
    wire_test_enum_defined_in_block_1_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_inbuilt_type_in_block_1(port_: MessagePort, a: i32, b: f32) {
    wire_test_inbuilt_type_in_block_1_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_test_list_in_block_1(
    port_: MessagePort,
    shared_structs: JsValue,
    strings: JsValue,
    nums: Box<[i32]>,
    weekdays: JsValue,
    struct_list: JsValue,
    enum_list: JsValue,
) {
    wire_test_list_in_block_1_impl(
        port_,
        shared_structs,
        strings,
        nums,
        weekdays,
        struct_list,
        enum_list,
    )
}

#[wasm_bindgen]
pub fn wire_test_method__method__EnumDefinedInBlock1(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__EnumDefinedInBlock1_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__StructDefinedInBlock1(
    port_: MessagePort,
    that: JsValue,
    message: String,
) {
    wire_test_method__method__StructDefinedInBlock1_impl(port_, that, message)
}

#[wasm_bindgen]
pub fn wire_test_method__method__StructOnlyForBlock1(
    port_: MessagePort,
    that: JsValue,
    message: String,
    num: u16,
) {
    wire_test_method__method__StructOnlyForBlock1_impl(port_, that, message, num)
}

#[wasm_bindgen]
pub fn wire_test_optional_string_in_block_1(port_: MessagePort, s: Option<String>, i: i32) {
    wire_test_optional_string_in_block_1_impl(port_, s, i)
}

#[wasm_bindgen]
pub fn wire_test_optional_string_in_sync_in_block_1(
    s: Option<String>,
    i: i32,
) -> support::WireSyncReturn {
    wire_test_optional_string_in_sync_in_block_1_impl(s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_in_block_1_for_1_and_2(
    port_: MessagePort,
    custom: JsValue,
    s: String,
    i: i32,
) {
    wire_test_shared_struct_in_block_1_for_1_and_2_impl(port_, custom, s, i)
}

#[wasm_bindgen]
pub fn wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1(
    name: String,
    score: f64,
) -> support::WireSyncReturn {
    wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1_impl(name, score)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__EnumDefinedInBlock1(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__EnumDefinedInBlock1_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__StructDefinedInBlock1(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__StructDefinedInBlock1_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_static_method__static_method__StructOnlyForBlock1(
    port_: MessagePort,
    message: String,
) {
    wire_test_static_method__static_method__StructOnlyForBlock1_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_test_string_in_block_1(port_: MessagePort, s: String, i: u64) {
    wire_test_string_in_block_1_impl(port_, s, i)
}

#[wasm_bindgen]
pub fn wire_test_string_in_sync_in_block_1(s: String, i: u64) -> support::WireSyncReturn {
    wire_test_string_in_sync_in_block_1_impl(s, i)
}

#[wasm_bindgen]
pub fn wire_test_struct_defined_in_block_1(port_: MessagePort, custom: JsValue) {
    wire_test_struct_defined_in_block_1_impl(port_, custom)
}

#[wasm_bindgen]
pub fn wire_test_unique_struct_1(port_: MessagePort, custom: JsValue, s: String, i: i8) {
    wire_test_unique_struct_1_impl(port_, custom, s, i)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<EnumDefinedInBlock1> for JsValue {
    fn wire2api(self) -> EnumDefinedInBlock1 {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => EnumDefinedInBlock1::Quit,
            1 => EnumDefinedInBlock1::Move {
                x: self_.get(1).wire2api(),
                y: self_.get(2).wire2api(),
            },
            2 => EnumDefinedInBlock1::Write(self_.get(1).wire2api()),
            3 => EnumDefinedInBlock1::ChangeColor(
                self_.get(1).wire2api(),
                self_.get(2).wire2api(),
                self_.get(3).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<Vec<EnumDefinedInBlock1>> for JsValue {
    fn wire2api(self) -> Vec<EnumDefinedInBlock1> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<StructDefinedInBlock1>> for JsValue {
    fn wire2api(self) -> Vec<StructDefinedInBlock1> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}

impl Wire2Api<StructDefinedInBlock1> for JsValue {
    fn wire2api(self) -> StructDefinedInBlock1 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        StructDefinedInBlock1 {
            name: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<StructOnlyForBlock1> for JsValue {
    fn wire2api(self) -> StructOnlyForBlock1 {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        StructOnlyForBlock1 {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
        }
    }
}
// Section: impl Wire2Api for JsValue

impl<T> Wire2Api<Option<T>> for JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<i8> for JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
