use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::sse::ty::rust_opaque::generate_maybe_unsafe;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_imports(&self) -> Option<Vec<Namespace>> {
        // To expose the `pub use`s inside that file
        Some(vec![self.mir.namespace.clone()])
    }

    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        let generate_io_web_impl = |target| -> WireRustOutputCode {
            ["increment", "decrement"]
                .iter()
                .map(|op| ExternFunc {
                    partial_func_name: format!(
                        "rust_arc_{op}_strong_count_{}",
                        self.mir.safe_ident()
                    ),
                    params: vec![ExternFuncParam {
                        name: "ptr".to_owned(),
                        rust_type: "*const std::ffi::c_void".to_owned(),
                        dart_type: "int".into(),
                    }
                    .clone()],
                    return_type: None,
                    body: generate_maybe_unsafe(
                        &format!(
                            "{}::<{}>::{op}_strong_count(ptr as _);",
                            self.mir.codec.arc_ty(),
                            &self.mir.inner.0.with_static_lifetime(),
                        ),
                        self.mir.codec.needs_unsafe_block(),
                    ),
                    target,
                    needs_ffigen: false,
                })
                .collect_vec()
                .into()
        };

        let common = if self.mir.codec == RustOpaqueCodecMode::Moi {
            format!(
                "flutter_rust_bridge::frb_generated_moi_arc_impl_value!({});\n",
                self.mir.inner.0.with_static_lifetime()
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
