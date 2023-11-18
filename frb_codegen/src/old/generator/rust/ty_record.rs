use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;

use super::ExternFuncCollector;
use super::TypeStructRefGenerator;

type_rust_generator_struct!(TypeRecordGenerator, IrTypeRecord);

impl TypeRustGeneratorTrait for TypeRecordGenerator<'_> {}
