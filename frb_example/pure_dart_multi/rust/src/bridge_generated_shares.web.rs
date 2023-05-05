use super::*;
// Section: wire functions

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
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

impl Wire2Api<SharedStructInAllBlocks> for JsValue {
    fn wire2api(self) -> SharedStructInAllBlocks {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            3,
            "Expected 3 elements, got {}",
            self_.length()
        );
        SharedStructInAllBlocks {
            id: self_.get(0).wire2api(),
            num: self_.get(1).wire2api(),
            name: self_.get(2).wire2api(),
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
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        SharedStructOnlyForSyncTest {
            default_score: self_.get(0).wire2api(),
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
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
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
