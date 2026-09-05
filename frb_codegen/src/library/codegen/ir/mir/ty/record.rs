use crate::codegen::ir::mir::ty::structure::MirTypeStructRef;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use itertools::Itertools;

crate::mir! {
pub struct MirTypeRecord {
    /// Refers to a virtual struct definition.
    pub inner: MirTypeStructRef,
    pub values: Box<[MirType]>,
}
}

impl MirTypeTrait for MirTypeRecord {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        for ty in self.values.iter() {
            ty.visit_types(f, mir_context)
        }
    }

    fn safe_ident(&self) -> String {
        self.inner.safe_ident()
    }

    fn rust_api_type(&self) -> String {
        let values = self
            .values
            .iter()
            .map(MirType::rust_api_type)
            .collect_vec()
            .join(",");
        format!("({values},)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::structure::MirStructIdent;
    use crate::utils::namespace::{Namespace, NamespacedName};

    /// Renders a one-element record as a Rust tuple with a trailing comma.
    #[test]
    fn record_rust_api_type_has_tuple_trailing_comma() {
        let record = MirTypeRecord {
            inner: MirTypeStructRef {
                ident: MirStructIdent(NamespacedName::new(
                    Namespace::new_raw("crate".to_owned()),
                    "Pair".to_owned(),
                )),
                is_exception: false,
            },
            values: vec![MirType::Primitive(MirTypePrimitive::I32)].into_boxed_slice(),
        };
        assert_eq!(record.rust_api_type(), "(i32,)");
    }
}
