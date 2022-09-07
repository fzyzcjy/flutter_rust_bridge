use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeSyncReturnGenerator, IrTypeSyncReturn);

impl TypeRustGeneratorTrait for TypeSyncReturnGenerator<'_> {
    fn wire2api_body(&self) -> Acc<Option<String>> {
        unimplemented!("SyncReturn generator for Rust: wire2api_body is not supported")
    }
}
