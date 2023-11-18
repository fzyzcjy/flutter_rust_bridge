use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target;
use crate::type_rust_generator_struct;

use super::generate_import;
use super::TypeGeneralListGenerator;

type_rust_generator_struct!(TypeOptionalListGenerator, IrTypeOptionalList);

impl TypeRustGeneratorTrait for TypeOptionalListGenerator<'_> {}
