use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypePrimitiveGenerator, IrTypePrimitive);

impl TypeRustGeneratorTrait for TypePrimitiveGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        "self".into()
    }
    fn wire2api_jsvalue(&self) -> Option<std::borrow::Cow<str>> {
        use IrTypePrimitive::*;
        Some(match self.ir {
            Bool => "self.is_truthy()".into(),
            I64 | U64 => "::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()".into(),
            _ => "self.unchecked_into_f64() as _".into(),
        })
    }
}
