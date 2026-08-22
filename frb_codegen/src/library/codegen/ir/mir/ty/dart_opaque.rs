use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeDartOpaque;
}

impl MirTypeTrait for MirTypeDartOpaque {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.get_delegate_rust().visit_types(f, mir_context);
        self.get_delegate_dart().visit_types(f, mir_context);
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
    pub(crate) fn get_delegate_rust(&self) -> MirType {
        MirType::Primitive(MirTypePrimitive::Usize)
    }

    pub(crate) fn get_delegate_dart(&self) -> MirType {
        MirType::Primitive(MirTypePrimitive::Isize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exposes architecture-specific Rust and Dart integer delegates.
    #[test]
    fn dart_opaque_uses_usize_and_isize_delegates() {
        let ty = MirTypeDartOpaque;

        assert_eq!(ty.safe_ident(), "DartOpaque");
        assert_eq!(ty.rust_api_type(), "flutter_rust_bridge::DartOpaque");
        assert!(matches!(
            ty.get_delegate_rust(),
            MirType::Primitive(MirTypePrimitive::Usize)
        ));
        assert!(matches!(
            ty.get_delegate_dart(),
            MirType::Primitive(MirTypePrimitive::Isize)
        ));
    }
}
