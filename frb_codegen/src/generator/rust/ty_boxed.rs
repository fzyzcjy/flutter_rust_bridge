use super::NO_PARAMS;

use crate::generator::rust::ty::*;
use crate::generator::rust::{generate_import, ExternFuncCollector};
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

// TODO: delete
// impl TypeBoxedGenerator<'_> {
//     fn as_struct_generator(&self) -> TypeStructRefGenerator {
//         let generator = TypeStructRefGenerator {
//             if let StructRef::Named { name, .. } = &self.ir.inner {
//                 ir: self.context.ir_file.get_struct(name).unwrap().clone(),
//             } else {
//                 unreachable!()
//             },
//             ir: self.ir.inner.inner.clone(),
//             context: self.context.clone(),
//         };
//     }
// }

impl TypeRustGeneratorTrait for TypeBoxedGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        let box_inner = self.ir.inner.as_ref();
        let exist_in_real_api = self.ir.exist_in_real_api;
        Acc::new(|target| match (target, self.ir.inner.as_ref()) {
            (Io, IrType::Primitive(_)) => Some(
                if exist_in_real_api {
                    "unsafe { support::box_from_leak_ptr(self) }"
                } else {
                    "unsafe { *support::box_from_leak_ptr(self) }"
                }
                .into(),
            ),
            (Io | Wasm, ir) if ir.is_array() => Some(format!(
                "Wire2Api::<{}>::wire2api(self).into()",
                box_inner.rust_api_type()
            )),
            (Io, _) => Some(format!(
                "let wrap = unsafe {{ support::box_from_leak_ptr(self) }};
                Wire2Api::<{}>::wire2api(*wrap).into()",
                box_inner.rust_api_type()
            )),
            _ => None,
        })
    }

    fn wire2api_jsvalue(&self) -> Option<std::borrow::Cow<str>> {
        (self.ir.exist_in_real_api).then(|| match &*self.ir.inner {
            IrType::Delegate(IrTypeDelegate::PrimitiveEnum { repr, .. }) => format!(
                "let ptr: Box<{}> = self.wire2api(); Box::new(ptr.wire2api())",
                repr.rust_api_type()
            )
            .into(),
            IrType::Delegate(IrTypeDelegate::Array(array)) => format!(
                "let vec: Vec<{}> = self.wire2api(); Box::new(support::from_vec_to_array(vec))",
                array.inner_rust_api_type()
            )
            .into(),
            _ => "Box::new(self.wire2api())".into(),
        })
    }

    fn wrapper_struct(&self) -> Option<String> {
        let src = TypeRustGenerator::new(
            *self.ir.inner.clone(),
            self.context.config,
            self.context.all_configs,
        );
        src.wrapper_struct()
    }

    fn self_access(&self, obj: String) -> String {
        format!("(*{obj})")
    }

    fn allocate_funcs(&self, collector: &mut ExternFuncCollector) -> Acc<Option<String>> {
        if self.ir.inner.is_array() {
            return Acc::default();
        }
        let func_name = format!("new_{}", self.ir.safe_ident());
        if self.ir.inner.is_primitive() {
            Acc {
                io: Some(collector.generate(
                    &func_name,
                    [(
                        format!("value: {}", self.ir.inner.rust_wire_type(Io)),
                        self.ir.inner.dart_wire_type(Io),
                    )],
                    Some(&format!("*mut {}", self.ir.inner.rust_wire_type(Io))),
                    "support::new_leak_box_ptr(value)",
                    Io,
                )),
                ..Default::default()
            }
        } else {
            Acc {
                io: Some(collector.generate(
                    &func_name,
                    NO_PARAMS,
                    Some(&[self.ir.rust_wire_modifier(Io), self.ir.rust_wire_type(Io)].concat()),
                    &format!(
                        "support::new_leak_box_ptr({}::new_with_null_ptr())",
                        self.ir.inner.rust_wire_type(Io)
                    ),
                    Io,
                )),
                ..Default::default()
            }
        }
    }

    fn imports(&self) -> Option<String> {
        generate_import(
            &self.ir.inner,
            self.context.config,
            self.context.all_configs,
        )
    }

    // TODO: delete
    // fn wire_struct_fields(&self) -> Option<Vec<String>> {
    // self.as_struct_generator().wire_struct_fields()
    // }
}
