use std::iter::FromIterator;

impl Target {
    #[inline]
    pub const fn call_convention(&self) -> &str {
        match self {
            Self::Io => "extern \"C\"",
            _ => "",
        }
    }
    #[inline]
    pub const fn extern_func_attr(&self) -> &str {
        match self {
            Self::Io => "#[no_mangle]",
            Self::Wasm => "#[wasm_bindgen]",
            _ => "",
        }
    }
}
