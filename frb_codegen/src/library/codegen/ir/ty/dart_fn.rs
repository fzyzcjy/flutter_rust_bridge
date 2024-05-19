use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use itertools::Itertools;

crate::ir! {
pub struct IrTypeDartFn {
    pub inputs: Vec<IrType>,
    pub output: Box<IrDartFnOutput>,
}

pub(crate) struct IrDartFnOutput {
    pub(crate) normal: IrType,
    pub(crate) error: IrType,
    /// Whether the error is provided to users, or error yields panic
    pub(crate) api_fallible: bool,
}
}

impl IrTypeTrait for IrTypeDartFn {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.get_delegate().visit_types(f, ir_context);

        for x in &self.inputs {
            x.visit_types(f, ir_context);
        }
        self.output.visit_types(f, ir_context);
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

impl IrTypeDartFn {
    pub(crate) fn get_delegate(&self) -> IrType {
        IrType::DartOpaque(IrTypeDartOpaque)
    }
}

impl IrDartFnOutput {
    pub(crate) fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.normal.visit_types(f, ir_context);
        if let Some(error) = &self.error {
            error.visit_types(f, ir_context);
        }
    }

    pub(crate) fn safe_ident(&self) -> String {
        format!(
            "{}_{}",
            self.normal.safe_ident(),
            self.error
                .map(|x| x.safe_ident())
                .unwrap_or("None".to_owned())
        )
    }

    pub(crate) fn rust_api_type(&self) -> String {
        if let Some(error) = &self.error {
            format!(
                "std::result::Result<{}, {}>",
                self.normal.rust_api_type(),
                error.rust_api_type()
            )
        } else {
            self.normal.rust_api_type()
        }
    }
}
