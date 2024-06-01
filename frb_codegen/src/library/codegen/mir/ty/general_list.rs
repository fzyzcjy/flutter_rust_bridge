use crate::codegen::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::mir::ty::primitive_list::MirTypePrimitiveList;
use crate::codegen::mir::ty::MirType::{GeneralList, PrimitiveList};
use crate::codegen::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::ir! {
pub struct MirTypeGeneralList {
    pub inner: Box<MirType>,
}
}

impl MirTypeTrait for MirTypeGeneralList {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl MirContext,
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

pub(crate) fn ir_list(inner: MirType, strict_dart_type: bool) -> MirType {
    match inner {
        // Since Dart doesn't have a boolean primitive list like `Uint8List`,
        // we need to convert `Vec<bool>` to a boolean general list in order to achieve the binding.
        MirType::Primitive(inner) if inner != MirTypePrimitive::Bool => {
            PrimitiveList(MirTypePrimitiveList {
                primitive: inner.clone(),
                strict_dart_type,
            })
        }
        _ => GeneralList(MirTypeGeneralList {
            inner: Box::new(inner),
        }),
    }
}
