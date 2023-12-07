use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::misc::consts::HANDLER_NAME;

impl<'a> WireRustGeneratorMiscTrait for DartOpaqueWireRustGenerator<'a> {
    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc {
            io: vec![ExternFunc {
                func_name: "dart_opaque_dart2rust_api2wire".into(),
                params: vec![ExternFuncParam {
                    name: "handle".to_owned(),
                    rust_type: "flutter_rust_bridge::for_generated::dart_sys::Dart_Handle".to_owned(),
                    dart_type: "NOT_USED".into(),
                }.clone()],
                return_type: Some("*const std::ffi::c_void".into()),
                body: format!("unsafe {{ flutter_rust_bridge::for_generated::dart_opaque_dart2rust_api2wire(&*{HANDLER_NAME}, handle) }}").into(),
                target: Target::Io,
            }].into(),
            ..Default::default()
        }
    }
}
