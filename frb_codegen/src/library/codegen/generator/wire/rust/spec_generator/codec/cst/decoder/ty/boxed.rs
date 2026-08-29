use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::misc::target::TargetOrCommon::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::misc::JS_VALUE;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegatePrimitiveEnum};
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl WireRustCodecCstGeneratorDecoderTrait for BoxedWireRustCodecCstGenerator<'_> {
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        let box_inner = self.mir.inner.as_ref();
        let exist_in_real_api = self.mir.exist_in_real_api;
        Acc::new(|target| {
            match (target, self.mir.inner.as_ref()) {
                (Io, MirType::Primitive(_)) => Some(format!(
                    "unsafe {{ {extra} flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }}",
                    extra = if exist_in_real_api { "" } else { "*" }
                )),
                (Io | Web, mir) if mir.is_array() => Some(format!(
                    "CstDecode::<{}>::cst_decode(self).into()",
                    box_inner.rust_api_type()
                )),
                (Io, _) => Some(format!(
                    "let wrap = unsafe {{ flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }};
                CstDecode::<{}>::cst_decode(*wrap).into()",
                    box_inner.rust_api_type()
                )),
                _ => None,
            }
        })
    }

    fn generate_impl_decode_jsvalue_body(&self) -> Option<std::borrow::Cow<'_, str>> {
        (self.mir.exist_in_real_api).then(|| match &*self.mir.inner {
            MirType::Delegate(MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum {
                                                               repr,
                                                               ..
                                                           })) => format!(
                "let ptr: Box<{}> = self.cst_decode(); Box::new(ptr.cst_decode())",
                repr.rust_api_type()
            )
                .into(),
            MirType::Delegate(MirTypeDelegate::Array(array)) => format!(
                "let vec: Vec<{}> = self.cst_decode(); Box::new(flutter_rust_bridge::for_generated::from_vec_to_array(vec))",
                array.inner().rust_api_type()
            )
                .into(),
            _ => "Box::new(self.cst_decode())".into(),
        })
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        if self.mir.inner.is_array() {
            return Acc::default();
        }
        let func_name = format!("cst_new_{}", self.mir.safe_ident());
        if self.mir.inner.is_primitive()
            || matches!(
                *self.mir.inner,
                MirType::RustOpaque(_)
                    | MirType::RustAutoOpaqueImplicit(_)
                    | MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(_))
                    | MirType::DartOpaque(_)
            )
        {
            Acc {
                io: ExternFunc {
                    partial_func_name: func_name,
                    params: vec![ExternFuncParam::new(
                        "value".to_owned(),
                        Target::Io,
                        &self.mir.inner,
                        self.context,
                    )],
                    return_type: Some(format!(
                        "*mut {}",
                        WireRustCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                            .rust_wire_type(Target::Io)
                    )),
                    body: "flutter_rust_bridge::for_generated::new_leak_box_ptr(value)".to_owned(),
                    target: Target::Io,
                    needs_ffigen: true,
                }
                .into(),
                ..Default::default()
            }
        } else {
            Acc {
                io: ExternFunc {
                    partial_func_name: func_name,
                    params: vec![],
                    return_type: Some(
                        [
                            self.rust_wire_modifier(Target::Io),
                            self.rust_wire_type(Target::Io),
                        ]
                            .concat(),
                    ),
                    body: format!(
                        "flutter_rust_bridge::for_generated::new_leak_box_ptr({}::new_with_null_ptr())",
                        WireRustCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                            .rust_wire_type(Target::Io)
                    ),
                    target: Target::Io,
                    needs_ffigen: true,
                }
                    .into(),
                ..Default::default()
            }
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if target == Target::Web && self.mir.inner.is_primitive() {
            JS_VALUE.into()
        } else {
            WireRustCodecCstGenerator::new(self.mir.inner.clone(), self.context)
                .rust_wire_type(target)
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        (target != Target::Web)
            || !is_js_value(&self.mir.inner)
                && !self.mir.inner.is_array()
                && !self.mir.inner.is_primitive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::boxed::MirTypeBoxed;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;

    /// Distinguishes API and auto-added boxed primitive decoding and allocation.
    #[test]
    fn boxed_decoder_covers_primitive_ownership_and_web_wire_types() {
        let pack = test_utils::pack();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let api_dart_config = test_utils::api_dart_config();
        let dart_context = test_utils::context(
            &pack,
            &wire_dart_config,
            &wire_rust_config,
            &api_dart_config,
        );
        let context = dart_context.as_wire_rust_context();

        for (exist_in_real_api, expected) in [
            (
                true,
                "unsafe {  flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }",
            ),
            (
                false,
                "unsafe { * flutter_rust_bridge::for_generated::box_from_leak_ptr(self) }",
            ),
        ] {
            let generator = BoxedWireRustCodecCstGenerator::new(
                MirTypeBoxed {
                    exist_in_real_api,
                    inner: Box::new(MirType::Primitive(MirTypePrimitive::I32)),
                },
                context,
            );
            assert_eq!(
                generator.generate_impl_decode_body().io.as_deref(),
                Some(expected)
            );
            assert_eq!(generator.rust_wire_type(Target::Web), JS_VALUE);
            assert!(!generator.rust_wire_is_pointer(Target::Web));
            assert_eq!(generator.generate_allocate_funcs().io.extern_funcs.len(), 1);
        }
    }
}
