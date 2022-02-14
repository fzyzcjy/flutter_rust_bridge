use crate::ir::ApiType::*;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct ApiTypeOptional {
    pub inner: Box<ApiType>,
}

impl ApiTypeOptional {
    pub fn new_prim(prim: ApiTypePrimitive) -> Self {
        Self {
            inner: Box::new(Boxed(ApiTypeBoxed {
                inner: Box::new(Primitive(prim)),
                exist_in_real_api: false,
            })),
        }
    }

    pub fn new_ptr(ptr: ApiType) -> Self {
        Self {
            inner: Box::new(ptr),
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if matches!(*boxed.inner, ApiType::Primitive(_)))
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

impl ApiTypeChild for ApiTypeOptional {
    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }
    fn rust_wire_type(&self) -> String {
        self.inner.rust_wire_type()
    }
    fn rust_api_type(&self) -> String {
        format!("Option<{}>", self.inner.rust_api_type())
    }
    fn dart_wire_type(&self) -> String {
        self.inner.dart_wire_type()
    }
    fn dart_api_type(&self) -> String {
        format!("{}?", self.inner.dart_api_type())
    }
    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}
