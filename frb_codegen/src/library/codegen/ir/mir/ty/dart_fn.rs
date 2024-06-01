use crate::codegen::ir::mir::ty::dart_opaque::MirTypeDartOpaque;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use itertools::Itertools;

crate::mir! {
pub struct MirTypeDartFn {
    pub inputs: Vec<MirType>,
    pub output: Box<MirDartFnOutput>,
}

pub(crate) struct MirDartFnOutput {
    pub(crate) normal: MirType,
    pub(crate) error: MirType,
    /// Whether the error is provided to users, or error yields panic
    pub(crate) api_fallible: bool,
}
}

impl MirTypeTrait for MirTypeDartFn {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.get_delegate().visit_types(f, mir_context);

        for x in &self.inputs {
            x.visit_types(f, mir_context);
        }
        self.output.visit_types(f, mir_context);
    }

    fn safe_ident(&self) -> String {
        format!(
            "DartFn_Inputs_{}_Output_{}",
            self.inputs.iter().map(|x| x.safe_ident()).join("_"),
            self.output.safe_ident()
        )
    }

    fn rust_api_type(&self) -> String {
        "DART_FN_RUST_API_TYPE_NOT_USED".to_owned()
        // format!(
        //     "impl Fn({}) -> flutter_rust_bridge::DartFnFuture<{}> + std::panic::UnwindSafe>",
        //     self.inputs.iter().map(|x| x.rust_api_type()).join(", "),
        //     self.output.rust_api_type()
        // )
    }
}

impl MirTypeDartFn {
    pub(crate) fn get_delegate(&self) -> MirType {
        MirType::DartOpaque(MirTypeDartOpaque)
    }
}

impl MirDartFnOutput {
    pub(crate) fn visit_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.normal.visit_types(f, mir_context);
        self.error.visit_types(f, mir_context);
    }

    pub(crate) fn safe_ident(&self) -> String {
        format!("{}_{}", self.normal.safe_ident(), self.error.safe_ident())
    }

    pub(crate) fn rust_api_type(&self) -> String {
        if self.api_fallible {
            format!(
                "std::result::Result<{}, {}>",
                self.normal.rust_api_type(),
                self.error.rust_api_type()
            )
        } else {
            self.normal.rust_api_type()
        }
    }
}
