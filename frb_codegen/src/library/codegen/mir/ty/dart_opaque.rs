use crate::codegen::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeDartOpaque;
}

impl MirTypeTrait for MirTypeDartOpaque {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl MirContext,
    ) {
        self.get_delegate().visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        "DartOpaque".to_owned()
    }

    fn rust_api_type(&self) -> String {
        "flutter_rust_bridge::DartOpaque".to_owned()
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        true
    }
}

impl MirTypeDartOpaque {
    pub(crate) fn get_delegate(&self) -> MirType {
        MirType::Primitive(MirTypePrimitive::Usize)
    }
}
