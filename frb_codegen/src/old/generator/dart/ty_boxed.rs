use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::IrType::{DartOpaque, Delegate, EnumRef, Primitive, RustOpaque, StructRef};
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

use super::gen_wire2api_chrono;

type_dart_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeDartGeneratorTrait for TypeBoxedGenerator<'_> {}
