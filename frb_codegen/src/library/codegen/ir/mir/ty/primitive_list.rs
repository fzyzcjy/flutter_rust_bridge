use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypePrimitiveList {
    pub primitive: MirTypePrimitive,
    pub strict_dart_type: bool,
}
}

impl MirTypeTrait for MirTypePrimitiveList {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        MirType::Primitive(self.primitive.clone()).visit_types(f, mir_context);

        if !self.strict_dart_type {
            MirType::PrimitiveList(MirTypePrimitiveList {
                strict_dart_type: true,
                ..self.clone()
            })
            .visit_types(f, mir_context);
        }
    }

    fn safe_ident(&self) -> String {
        format!(
            "list_prim_{}_{}",
            self.primitive.safe_ident(),
            if self.strict_dart_type {
                "strict"
            } else {
                "loose"
            }
        )
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Delegates primitive-list identity and Rust type formatting.
    #[test]
    fn primitive_list_contracts_are_stable() {
        let loose = MirTypePrimitiveList {
            primitive: MirTypePrimitive::I32,
            strict_dart_type: false,
        };
        assert_eq!(loose.safe_ident(), "list_prim_i_32_loose");
        assert_eq!(loose.rust_api_type(), "Vec<i32>");
    }
}
