use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeDynamic;
}

impl MirTypeTrait for MirTypeDynamic {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        _f: &mut F,
        _mir_context: &impl MirContext,
    ) {
    }

    fn safe_ident(&self) -> String {
        "dartabi".to_owned()
    }

    fn rust_api_type(&self) -> String {
        "flutter_rust_bridge::for_generated::DartAbi".to_owned()
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps the dynamic ABI identity and clone semantics stable.
    #[test]
    fn dynamic_type_uses_the_dart_abi_contract() {
        let ty = MirTypeDynamic;

        assert_eq!(ty.safe_ident(), "dartabi");
        assert_eq!(
            ty.rust_api_type(),
            "flutter_rust_bridge::for_generated::DartAbi"
        );
        assert!(ty.cloned_getter_semantics_reasonable());
    }
}
