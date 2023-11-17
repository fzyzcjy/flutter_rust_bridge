use crate::generator::rust::ty::*;
use crate::generator::rust::{
    generate_list_allocate_func, ExternFuncCollector, TypeGeneralListGenerator,
};
use crate::ir::*;
use crate::target::{Acc, Target};
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

use super::generate_impl_into_into_dart;

type_rust_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeRustGeneratorTrait for TypeDelegateGenerator<'_> {
    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        _: BlockIndex,
    ) -> Acc<Option<String>> {
        match &self.ir {
            list @ IrTypeDelegate::StringList => Acc {
                io: Some(generate_list_allocate_func(
                    collector,
                    &self.ir.safe_ident(),
                    list,
                    &list.get_delegate(),
                    self.context.config.block_index,
                )),
                ..Default::default()
            },
            _ => Default::default(),
        }
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<std::borrow::Cow<str>> {
        Some(match &self.ir {
            IrTypeDelegate::String => {
                "self.as_string().expect(\"non-UTF-8 string, or not a string\")".into()
            }
            IrTypeDelegate::PrimitiveEnum { repr, .. } => format!(
                "(self.unchecked_into_f64() as {}).wire2api()",
                repr.rust_wire_type(Target::Wasm)
            )
            .into(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer(self.wire2api())".into()
            }
            IrTypeDelegate::Time(_) => "Wire2Api::<i64>::wire2api(self).wire2api()".into(),
            IrTypeDelegate::TimeList(_) =>
                "self.unchecked_into::<js_sys::BigInt64Array>().to_vec().into_iter().map(Wire2Api::wire2api).collect()".into(),
            IrTypeDelegate::Uuid | IrTypeDelegate::Uuids => {
                "self.unchecked_into::<js_sys::Uint8Array>().to_vec().into_boxed_slice().wire2api()"
                    .into()
            }
            IrTypeDelegate::Array(array) => format!(
                "let vec: Vec<{}> = self.wire2api(); support::from_vec_to_array(vec)",
                array.inner().rust_api_type()
            )
            .into(),
            _ => return None,
        })
    }

    fn imports(&self) -> Option<String> {
        forward_delegate_primitive_enum!(self, imports(), None)
    }
}
