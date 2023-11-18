use itertools::Itertools;

use crate::generator::dart::dart_comments;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::misc::dart_maybe_implements_exception;

type_dart_generator_struct!(TypeEnumRefGenerator, IrTypeEnumRef);

impl TypeDartGeneratorTrait for TypeEnumRefGenerator<'_> {}
