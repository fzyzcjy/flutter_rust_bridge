use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::WireDartCodecCstGenerator;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use itertools::Itertools;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ExternFunc {
    pub(crate) partial_func_name: String,
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
    pub(crate) fn generate(&self, name_prefix: &str) -> String {
        let call_convention = match self.target {
            Target::Io => "extern \"C\"",
            Target::Web => "",
        };
        let attribute = match self.target {
            Target::Io => "#[no_mangle]",
            Target::Web => "#[wasm_bindgen]",
        };
        let ExternFunc {
            partial_func_name,
            body,
            ..
        } = self;

        let func_name = format!("{name_prefix}{partial_func_name}");

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
    // TODO move this func to cst-specific
    pub(crate) fn new(
        name: String,
        target: Target,
        ty: &IrType,
        context: WireRustCodecCstGeneratorContext,
    ) -> Self {
        let rust_gen = WireRustCodecCstGenerator::new(ty.clone(), context);
        let dart_gen = WireDartCodecCstGenerator::new(ty.clone(), context.as_wire_dart_context());

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

// TODO maybe move
#[derive(Clone, Debug, Serialize)]
pub(crate) struct ExternClass {
    pub partial_name: String,
    pub mode: ExternClassMode,
    pub body: String,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) enum ExternClassMode {
    Struct,
    Union,
}

impl ExternClass {
    pub(crate) fn generate(&self, name_prefix: &str) -> String {
        let ExternClass {
            partial_name,
            mode,
            body,
        } = self;

        let mode = match mode {
            ExternClassMode::Struct => "struct",
            ExternClassMode::Union => "union",
        };

        let name = format!("{name_prefix}{partial_name}");

        format!("#[repr(C)] #[derive(Clone, Copy)] pub {mode} {name} {{ {body} }}")
    }
}
