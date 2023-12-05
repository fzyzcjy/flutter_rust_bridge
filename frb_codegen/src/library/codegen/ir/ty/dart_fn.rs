use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use itertools::Itertools;

crate::ir! {
pub struct IrTypeDartFn {
    pub inputs: Vec<IrType>,
    pub output: Box<IrType>,
}
}

impl IrTypeTrait for IrTypeDartFn {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
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
        "TODO_rust_api_type".into()
    }
}
