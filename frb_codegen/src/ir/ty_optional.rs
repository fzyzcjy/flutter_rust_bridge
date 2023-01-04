use crate::ir::IrType::*;
use crate::ir::*;
use crate::target::Target;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrTypeOptional {
    pub inner: Box<IrType>,
}

impl IrTypeOptional {
    pub fn new(ptr: IrType) -> Self {
        Self {
            inner: Box::new(ptr),
        }
    }

    pub fn new_boxed(inner: IrType) -> Self {
        Self {
            inner: Box::new(Boxed(IrTypeBoxed {
                exist_in_real_api: false,
                inner: Box::new(inner),
            })),
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if ! boxed.exist_in_real_api && boxed.inner.is_primitive())
    }

    pub fn is_boxed_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if boxed.exist_in_real_api && boxed.inner.is_primitive())
    }

    pub fn is_list(&self) -> bool {
        matches!(&*self.inner, GeneralList(_) | PrimitiveList(_))
    }

    pub fn is_delegate(&self) -> bool {
        matches!(&*self.inner, Delegate(_))
    }

    pub fn needs_initialization(&self) -> bool {
        !(self.is_primitive() || self.is_delegate())
    }
}

impl IrTypeTrait for IrTypeOptional {
    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if self.inner.rust_wire_is_pointer(target)
            || target.is_wasm()
                && (self.inner.is_js_value() || self.is_primitive() || self.is_boxed_primitive())
        {
            self.inner.rust_wire_type(target)
        } else {
            format!("Option<{}>", self.inner.rust_wire_type(target))
        }
    }
    fn rust_api_type(&self) -> String {
        format!("Option<{}>", self.inner.rust_api_type())
    }
    fn dart_wire_type(&self, target: Target) -> String {
        if target.is_wasm() {
            format!("{}?", self.inner.dart_wire_type(target))
        } else {
            self.inner.dart_wire_type(target)
        }
    }
    fn dart_api_type(&self) -> String {
        format!("{}?", self.inner.dart_api_type())
    }
    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        !target.is_wasm() || self.inner.rust_wire_is_pointer(target)
    }

    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.inner.visit_types(f, ir_file);
    }
}
