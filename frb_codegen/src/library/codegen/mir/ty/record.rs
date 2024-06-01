use crate::codegen::mir::ty::structure::MirTypeStructRef;
use crate::codegen::mir::ty::{MirContext, MirType, MirTypeTrait};
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
        ir_context: &impl MirContext,
    ) {
        for ty in self.values.iter() {
            ty.visit_types(f, ir_context)
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
