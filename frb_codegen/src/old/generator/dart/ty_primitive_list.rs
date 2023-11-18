use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypePrimitiveListGenerator, IrTypePrimitiveList);

impl TypeDartGeneratorTrait for TypePrimitiveListGenerator<'_> {}
