use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGenerator;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ExternFunc {
    pub(crate) func_name: String,
    pub(crate) params: Vec<ExternFuncParam>,
    pub(crate) return_type: Option<String>,
    pub(crate) body: String,
    pub(crate) target: Target,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ExternFuncParam {
    pub(crate) name: String,
    pub(crate) rust_type: String,
    pub(crate) dart_type: String,
}

impl ExternFunc {
    pub(crate) fn generate(&self) -> String {
        let call_convention = match self.target {
            Target::Io => "extern \"C\"",
            Target::Wasm => "",
        };
        let attribute = match self.target {
            Target::Io => "#[no_mangle]",
            Target::Wasm => "#[wasm_bindgen]",
        };
        let ExternFunc {
            func_name, body, ..
        } = self;

        format!(
            r#"
                {attribute}
                pub {call_convention} fn {func_name}({}) {} {{
                    {body}
                }}
            "#,
            self.params
                .iter()
                .map(|param| param.rust_name_and_type())
                .join(", "),
            self.return_type
                .as_ref()
                .map_or("".to_owned(), |r| format!("-> {r}")),
        )
    }
}

impl ExternFuncParam {
    pub(crate) fn new(
        name: String,
        target: Target,
        ty: &IrType,
        context: WireRustGeneratorContext,
    ) -> Self {
        let rust_gen = WireRustGenerator::new(ty.clone(), context);
        let dart_gen = WireDartGenerator::new(ty.clone(), context.as_wire_dart_context());

        Self {
            name,
            rust_type: format!(
                "{}{}",
                rust_gen.rust_wire_modifier(target),
                rust_gen.rust_wire_type(target)
            ),
            dart_type: dart_gen.dart_wire_type(target),
        }
    }

    pub(crate) fn rust_name_and_type(&self) -> String {
        format!("{}: {}", self.name, self.rust_type)
    }
}
