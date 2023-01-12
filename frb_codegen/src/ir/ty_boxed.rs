use crate::ir::*;
use crate::target::Target;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrTypeBoxed {
    /// if false, means that we automatically add it when transforming it - it does not exist in real api.
    pub exist_in_real_api: bool,
    pub inner: Box<IrType>,
}

impl IrTypeTrait for IrTypeBoxed {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.inner.visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        format!(
            "box_{}{}",
            if self.exist_in_real_api {
                ""
            } else {
                "autoadd_"
            },
            self.inner.safe_ident()
        )
    }

    fn dart_api_type(&self) -> String {
        self.inner.dart_api_type()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Wasm => {
                if self.inner.is_js_value() || self.inner.is_array() || self.inner.is_primitive() {
                    self.inner.dart_wire_type(target)
                } else {
                    format!("int /* *{} */", self.inner.rust_wire_type(target))
                }
            }
            Target::Io => {
                if self.inner.is_array() {
                    return self.inner.dart_wire_type(Target::Io);
                }
                let wire_type = self
                    .inner
                    .as_primitive()
                    .map(|prim| prim.dart_native_type().to_owned())
                    .unwrap_or_else(|| self.inner.dart_wire_type(target));
                format!("ffi.Pointer<{}>", wire_type)
            }
            Target::Common => "".into(),
        }
    }

    fn rust_api_type(&self) -> String {
        if self.exist_in_real_api {
            format!("Box<{}>", self.inner.rust_api_type())
        } else {
            self.inner.rust_api_type()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if target.is_wasm() && self.inner.is_primitive() {
            "JsValue".into()
        } else {
            self.inner.rust_wire_type(target)
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        !target.is_wasm()
            || !self.inner.is_js_value() && !self.inner.is_array() && !self.inner.is_primitive()
    }
}
