use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::misc::consts::HANDLER_NAME;

impl<'a> WireRustGeneratorMiscTrait for DartOpaqueWireRustGenerator<'a> {
    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc::new(|target| match target {
            TargetOrCommon::Io | TargetOrCommon::Wasm => {
                let target: Target = target.try_into().unwrap();
                let func = ExternFunc {
                        func_name: "dart_opaque_dart2rust_encode".into(),
                        params: vec![ExternFuncParam {
                            name: "handle".to_owned(),
                            rust_type: match target {
                                Target::Io => "flutter_rust_bridge::for_generated::dart_sys::Dart_Handle",
                                Target::Wasm => "flutter_rust_bridge::for_generated::wasm_bindgen::JsValue",
                            }.into(),
                            dart_type: "Object".into(),
                        }],
                        return_type: Some(match target {
                            Target::Io => "*const std::ffi::c_void",
                            Target::Wasm => "usize",
                        }.into()),
                        body: format!("unsafe {{ flutter_rust_bridge::for_generated::dart_opaque_dart2rust_encode(&*{HANDLER_NAME}, handle) as _ }}"),
                        target,
                    };
                vec![func].into()
            }
            TargetOrCommon::Common => Default::default(),
        })
    }
}
