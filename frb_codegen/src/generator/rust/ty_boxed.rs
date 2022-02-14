use crate::generator::rust::ty::TypeRustGeneratorTrait;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::IrType::Primitive;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeBoxedGenerator(pub IrTypeBoxed);

impl TypeRustGeneratorTrait for TypeBoxedGenerator {
    fn wire2api_body(&self) -> String {
        let IrTypeBoxed {
            inner: box_inner,
            exist_in_real_api,
        } = &self.0;
        match (box_inner.as_ref(), exist_in_real_api) {
            (IrType::Primitive(_), false) => "unsafe { *support::box_from_leak_ptr(self) }".into(),
            (IrType::Primitive(_), true) => "unsafe { support::box_from_leak_ptr(self) }".into(),
            _ => {
                "let wrap = unsafe { support::box_from_leak_ptr(self) }; (*wrap).wire2api().into()"
                    .into()
            }
        }
    }

    fn allocate_funcs(&self, collector: &ExternFuncCollector) -> String {
        match &*self.0.inner {
            Primitive(prim) => self.extern_func_collector.generate(
                &format!("new_{}", self.0.safe_ident()),
                &[&format!("value: {}", prim.rust_wire_type())],
                Some(&format!("*mut {}", prim.rust_wire_type())),
                "support::new_leak_box_ptr(value)",
            ),
            inner => self.extern_func_collector.generate(
                &format!("new_{}", self.0.safe_ident()),
                &[],
                Some(&[self.0.rust_wire_modifier(), ty.rust_wire_type()].concat()),
                &format!(
                    "support::new_leak_box_ptr({}::new_with_null_ptr())",
                    inner.rust_wire_type()
                ),
            ),
        }
    }

    fn imports(&self) -> Option<String> {
        self.generate_import(&self.0.inner, ir_file)
    }
}
