use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use convert_case::{Case, Casing};

impl<'a> WireDartGeneratorMiscTrait for RustOpaqueWireDartGenerator<'a> {
    fn generate_extra_functions(&self) -> Option<Acc<WireDartOutputCode>> {
        Some(
            vec![
                self.generate_rust_arc_modify_strong_count("increment"),
                self.generate_rust_arc_modify_strong_count("decrement"),
                self.generate_function_pointer(),
            ]
            .into_iter()
            .collect::<Acc<Vec<WireDartOutputCode>>>()
            .map(|x, _| x.into_iter().fold(Default::default(), |a, b| a + b)),
        )
    }
}

impl<'a> RustOpaqueWireDartGenerator<'a> {
    fn generate_rust_arc_modify_strong_count(&self, op_name: &str) -> Acc<WireDartOutputCode> {
        let ty_dart_api_type =
            ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                .dart_api_type();
        let op_name_pascal = op_name.to_case(Case::Pascal);
        let safe_ident = self.ir.safe_ident();

        let definition = format!(
            "RustArc{op_name_pascal}StrongCountFnType get rust_arc_{op_name}_strong_count_{ty_dart_api_type}"
        );

        Acc {
            common: WireDartOutputCode {
                api_body: format!("{definition};\n\n"),
                api_impl_body: format!(
                    "{definition} => wire.rust_arc_{op_name}_strong_count_{safe_ident};\n\n"
                ),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn generate_function_pointer(&self) -> Acc<WireDartOutputCode> {
        let ty_dart_api_type =
            ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                .dart_api_type();
        let ty_safe_ident = self.ir.safe_ident();
        let getter_name = format!("rust_arc_decrement_strong_count_{ty_dart_api_type}Ptr");

        let generate_platform_impl = |ptr_name: &str| WireDartOutputCode {
            api_impl_body: format!(
                "CrossPlatformFinalizerArg get {getter_name} => {ptr_name};\n\n",
            ),
            ..Default::default()
        };

        Acc {
            common: WireDartOutputCode {
                api_body: format!("CrossPlatformFinalizerArg get {getter_name};\n\n"),
                ..Default::default()
            },
            io: generate_platform_impl(&format!(
                "wire._rust_arc_decrement_strong_count_{ty_safe_ident}Ptr"
            )),
            wasm: generate_platform_impl(&format!(
                "wire.rust_arc_decrement_strong_count_{ty_safe_ident}"
            )),
        }
    }
}
