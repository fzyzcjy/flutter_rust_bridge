use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorMiscTrait for DartOpaqueWireRustGenerator<'a> {
    // fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
    //     Acc::new(|target| match target {
    //         TargetOrCommon::Io | TargetOrCommon::Web => {
    //             let target: Target = target.try_into().unwrap();
    //             let func = ExternFunc {
    //                 func_name: "dart_opaque_dart2rust_encode".into(),
    //                 params: vec![
    //                     ExternFuncParam {
    //                         name: "handle".to_owned(),
    //                         rust_type: match target {
    //                             Target::Io => "flutter_rust_bridge::for_generated::dart_sys::Dart_Handle",
    //                             Target::Web => "flutter_rust_bridge::for_generated::wasm_bindgen::JsValue",
    //                         }.into(),
    //                         dart_type: "Object".into(),
    //                     },
    //                     ExternFuncParam {
    //                         name: "dart_handler_port".to_owned(),
    //                         rust_type: match target {
    //                             Target::Io => "i64",
    //                             Target::Web => "flutter_rust_bridge::for_generated::MessagePort",
    //                         }.into(),
    //                         dart_type: "Object".into(),
    //                     },
    //                 ],
    //                 return_type: Some(match target {
    //                     Target::Io => "*const std::ffi::c_void",
    //                     Target::Web => "usize",
    //                 }.into()),
    //                 body: format!("unsafe {{ flutter_rust_bridge::for_generated::dart_opaque_dart2rust_encode(&*{HANDLER_NAME}, handle, dart_handler_port) as _ }}"),
    //                 target,
    //             };
    //             vec![func].into()
    //         }
    //         TargetOrCommon::Common => Default::default(),
    //     })
    // }
}
