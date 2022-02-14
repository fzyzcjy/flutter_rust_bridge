use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::generator::rust::{ExternFuncCollector, TypeGeneralListGenerator};
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeDelegateGenerator(IrTypeDelegate);

impl TypeRustGeneratorTrait for TypeDelegateGenerator {
    fn wire2api_body(&self) -> String {
        match &self.0 {
            IrTypeDelegate::String => "let vec: Vec<u8> = self.wire2api();
            String::from_utf8_lossy(&vec).into_owned()"
                .into(),
            IrTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".into(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer(self.wire2api())".into()
            }
            IrTypeDelegate::StringList => TypeGeneralListGenerator::WIRE2API_BODY.to_string(),
        }
    }

    fn wire_struct_fields(&self) -> Vec<String> {
        match &self.0 {
            ty @ IrTypeDelegate::StringList => vec![
                format!("ptr: *mut *mut {}", ty.get_delegate().rust_wire_type()),
                "len: i32".to_owned(),
            ],
            _ => "".to_string(),
        }
    }

    fn allocate_funcs(&self, collector: &ExternFuncCollector) -> String {
        match &self.0 {
            list @ IrTypeDelegate::StringList => {
                self.generate_list_allocate_func(&ty.safe_ident(), list, &list.get_delegate())
            }
            _ => "".to_string(),
        }
    }
}
