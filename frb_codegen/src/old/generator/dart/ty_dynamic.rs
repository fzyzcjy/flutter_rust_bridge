use crate::{ir::IrTypeDynamic, target::Acc, type_dart_generator_struct};

use super::TypeDartGeneratorTrait;

type_dart_generator_struct!(TypeDynamicGenerator, IrTypeDynamic);

impl TypeDartGeneratorTrait for TypeDynamicGenerator<'_> {}
