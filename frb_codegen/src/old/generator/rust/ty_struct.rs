use crate::generator::rust::generate_impl_into_into_dart;
use crate::generator::rust::ty::*;
use crate::generator::rust::ExternFuncCollector;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;

type_rust_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeRustGeneratorTrait for TypeStructRefGenerator<'_> {}
