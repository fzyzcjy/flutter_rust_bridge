use crate::codegen::generator::wire::rust::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::base::*;

impl<'a> WireRustGeneratorApi2wireTrait for DelegateWireRustGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        if let IrTypeDelegate::PrimitiveEnum { ir, .. } = &self.ir {
            let src = ir.get(self.context.ir_pack);
            let (name, self_path): (&str, &str) = match &src.wrapper_name {
                Some(wrapper) => (wrapper, &src.name),
                None => (&src.name, "Self"),
            };
            let self_ref = self.self_access("self".to_owned());
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| format!("{}::{} => {},", self_path, variant.name, idx))
                .collect_vec()
                .join("\n");
            let into_into_dart = get_into_into_dart(&src.name, src.wrapper_name.as_ref());
            return format!(
                "impl support::IntoDart for {name} {{
                    fn into_dart(self) -> support::DartAbi {{
                        match {self_ref} {{
                            {variants}
                        }}.into_dart()
                    }}
                }}
                impl support::IntoDartExceptPrimitive for {name} {{}}
                {into_into_dart}
                "
            );
        }
        "".into()
    }
}
