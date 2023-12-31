use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::IrType::{GeneralList, PrimitiveList};
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeGeneralList {
    pub inner: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeGeneralList {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.inner.visit_types(f, ir_context);
    }

    fn safe_ident(&self) -> String {
        format!("list_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.inner.rust_api_type())
    }
}

pub(crate) fn ir_list(inner: IrType) -> IrType {
    match inner {
        IrType::Primitive(inner) => {
            // Since Dart doesn't have a boolean primitive list like `Uint8List`,
            // we need to convert `Vec<bool>` to a boolean general list in order to achieve the binding.
            if inner == IrTypePrimitive::Bool {
                GeneralList(IrTypeGeneralList {
                    inner: Box::new(IrType::Primitive(IrTypePrimitive::Bool)),
                })
            } else {
                PrimitiveList(IrTypePrimitiveList {
                    primitive: inner.clone(),
                })
            }
        }
    }
}
