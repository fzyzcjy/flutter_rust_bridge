use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeBoxed {
    /// if false, means that we automatically add it when transforming it - it does not exist in real api.
    pub exist_in_real_api: bool,
    pub inner: Box<MirType>,
}
}

impl MirTypeTrait for MirTypeBoxed {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.inner.visit_types(f, mir_context);
    }

    fn safe_ident(&self) -> String {
        format!(
            "box_{}{}",
            if self.exist_in_real_api {
                ""
            } else {
                "autoadd_"
            },
            self.inner.safe_ident()
        )
    }

    fn rust_api_type(&self) -> String {
        if self.exist_in_real_api {
            format!("Box<{}>", self.inner.rust_api_type())
        } else {
            self.inner.rust_api_type()
        }
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        self.inner.cloned_getter_semantics_reasonable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

    /// Distinguishes real API boxes from automatically inserted wrappers.
    #[test]
    fn boxed_type_formats_real_and_automatic_boxes_differently() {
        let real = MirTypeBoxed {
            exist_in_real_api: true,
            inner: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
        };
        let automatic = MirTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
        };

        assert_eq!(real.safe_ident(), "box_i_32");
        assert_eq!(real.rust_api_type(), "Box<i32>");
        assert_eq!(automatic.safe_ident(), "box_autoadd_i_32");
        assert_eq!(automatic.rust_api_type(), "i32");
    }
}
