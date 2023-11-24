use crate::codegen::generator::wire::rust::spec_generator::api2wire::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use crate::forward_delegate_primitive_enum;
use itertools::Itertools;

impl<'a> WireRustGeneratorApi2wireTrait for DelegateWireRustGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        if let IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) = &self.ir {
            let src = ir.get(self.context.ir_pack);
            let (name, self_path): (&str, String) = match &src.wrapper_name {
                Some(wrapper) => (&wrapper.name, src.name.rust_style()),
                None => (&src.name.name, "Self".into()),
            };
            let self_ref = self.generate_access_object_core("self".to_owned());
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| format!("{}::{} => {},", self_path, variant.name, idx))
                .collect_vec()
                .join("\n");
            let into_into_dart = generate_impl_into_into_dart(&src.name, &src.wrapper_name);
            return Some(format!(
                "impl flutter_rust_bridge::support::IntoDart for {name} {{
                    fn into_dart(self) -> flutter_rust_bridge::support::DartAbi {{
                        match {self_ref} {{
                            {variants}
                        }}.into_dart()
                    }}
                }}
                impl flutter_rust_bridge::support::IntoDartExceptPrimitive for {name} {{}}
                {into_into_dart}
                "
            ));
        }
        None
    }

    fn generate_access_object_core(&self, obj: String) -> String {
        forward_delegate_primitive_enum!(self, generate_access_object_core(obj), obj)
    }
}
