use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_imports(&self) -> Option<Vec<String>> {
        // To expose the `pub use`s inside that file
        Some(vec![format!("use {}::*;", self.ir.namespace.joined_path)])
    }

    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        let generate_io_web_impl = |target| -> WireRustOutputCode {
            ["increment", "decrement"].iter()
                .map(|op|
                     ExternFunc {
                         partial_func_name: format!("rust_arc_{op}_strong_count_{}", self.ir.safe_ident()),
                         params: vec![ExternFuncParam {
                             name: "ptr".to_owned(),
                             rust_type: "*const std::ffi::c_void".to_owned(),
                             dart_type: "dynamic".into(),
                         }.clone()],
                         return_type: None,
                         body: format!(
                             "unsafe {{ <flutter_rust_bridge::{}RustOpaqueCodec as flutter_rust_bridge::for_generated::BaseRustOpaqueCodec<{}>>::Arc::{op}_strong_count(ptr as _); }}",
                             self.ir.codec.to_string(),
                             &self.ir.inner.rust_api_type(),
                         ),
                         target,
                     },
                )
                .collect_vec()
                .into()
        };

        let common = if self.ir.codec == RustOpaqueCodecMode::Moi {
            format!(
                "flutter_rust_bridge::frb_generated_map_based_arc_impl_value!({});\n",
                self.ir.rust_api_type()
            )
        } else {
            "".to_owned()
        };

        Acc {
            io: generate_io_web_impl(Target::Io),
            web: generate_io_web_impl(Target::Web),
            common: common.into(),
        }
    }
}
