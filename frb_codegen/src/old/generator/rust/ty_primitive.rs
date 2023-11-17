use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypePrimitiveGenerator, IrTypePrimitive);

impl TypeRustGeneratorTrait for TypePrimitiveGenerator<'_> {}
