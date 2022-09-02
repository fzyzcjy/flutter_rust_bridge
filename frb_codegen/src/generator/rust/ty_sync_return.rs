use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeSyncReturnGenerator, IrTypeSyncReturn);

impl TypeRustGeneratorTrait for TypeSyncReturnGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
        Some("/*unsupported*/".into())
    }
}
