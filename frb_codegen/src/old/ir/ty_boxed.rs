use crate::ir::*;
use crate::target::Target;

impl IrTypeTrait for IrTypeBoxed {
    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Wasm => {
                if is_js_value(self.inner) || self.inner.is_array() || self.inner.is_primitive() {
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
                format!("ffi.Pointer<{wire_type}>")
            }
            Target::Common => unreachable!(),
        }
    }
}
