use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_get_app_settings_to_api2(port_: MessagePort) {
    wire_get_app_settings_to_api2_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_fallible_app_settings_to_api2(port_: MessagePort) {
    wire_get_fallible_app_settings_to_api2_impl(port_)
}

#[wasm_bindgen]
pub fn wire_is_app_embedded_in_api2(port_: MessagePort, app_settings: JsValue) {
    wire_is_app_embedded_in_api2_impl(port_, app_settings)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<ApplicationEnv> for JsValue {
    fn wire2api(self) -> ApplicationEnv {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        ApplicationEnv {
            vars: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<ApplicationEnvVar> for JsValue {
    fn wire2api(self) -> ApplicationEnvVar {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        ApplicationEnvVar(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}

impl Wire2Api<ApplicationSettings> for JsValue {
    fn wire2api(self) -> ApplicationSettings {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            5,
            "Expected 5 elements, got {}",
            self_.length()
        );
        ApplicationSettings {
            name: self_.get(0).wire2api(),
            version: self_.get(1).wire2api(),
            mode: self_.get(2).wire2api(),
            env: self_.get(3).wire2api(),
            env_optional: self_.get(4).wire2api(),
        }
    }
}

impl Wire2Api<Vec<ApplicationEnvVar>> for JsValue {
    fn wire2api(self) -> Vec<ApplicationEnvVar> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Option<ApplicationEnv>> for JsValue {
    fn wire2api(self) -> Option<ApplicationEnv> {
        (!self.is_undefined() && !self.is_null()).then(|| self.wire2api())
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
impl Wire2Api<ApplicationMode> for JsValue {
    fn wire2api(self) -> ApplicationMode {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<bool> for JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<Box<ApplicationEnv>> for JsValue {
    fn wire2api(self) -> Box<ApplicationEnv> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
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
