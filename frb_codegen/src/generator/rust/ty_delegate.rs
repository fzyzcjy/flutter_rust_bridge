use crate::generator::rust::ty::*;
use crate::generator::rust::{
    generate_list_allocate_func, ExternFuncCollector, TypeGeneralListGenerator,
};
use crate::ir::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeRustGeneratorTrait for TypeDelegateGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
        Some(match &self.ir {
            IrTypeDelegate::String => "let vec: Vec<u8> = self.wire2api();
            String::from_utf8_lossy(&vec).into_owned()"
                .into(),
            IrTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".into(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer(self.wire2api())".into()
            }
            IrTypeDelegate::StringList => TypeGeneralListGenerator::WIRE2API_BODY.to_string(),
            IrTypeDelegate::ArrayPrimitive { primitive: _, len } => format!(
                "use std::convert::TryInto;

            let vec = unsafe {{
                let wrap = support::box_from_leak_ptr(self);
                support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }};
            if vec.len() == {len} {{
                Some(vec.try_into().unwrap())
            }} else {{
                None
            }}
            "
            ),
            IrTypeDelegate::ArrayGeneral {
                ir_type_general_list,
                len,
            } => format!(
                "use std::convert::TryInto;

            let vec = unsafe {{
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
            }};
            let vec = vec.iter().cloned()
                .map(Wire2Api::wire2api)
                .collect::<Vec<{}>>();
            if vec.len() == {len} {{
                vec.try_into().unwrap()
            }} else {{
                panic!(\"wuuuuu\");
                [{}::default(); {len}]
            }}
            ",
                ir_type_general_list.inner.rust_api_type(),
                ir_type_general_list.inner.rust_api_type(),
            ),
        })
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        match &self.ir {
            ty @ IrTypeDelegate::StringList => Some(vec![
                format!("ptr: *mut *mut {}", ty.get_delegate().rust_wire_type()),
                "len: i32".to_owned(),
            ]),
            _ => None,
        }
    }

    fn allocate_funcs(&self, collector: &mut ExternFuncCollector) -> String {
        match &self.ir {
            list @ IrTypeDelegate::StringList => generate_list_allocate_func(
                collector,
                &self.ir.safe_ident(),
                list,
                &list.get_delegate(),
            ),
            _ => "".to_string(),
        }
    }
}
