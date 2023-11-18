use crate::generator::dart::ty::*;
use crate::generator::dart::{dart_comments, dart_metadata};
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::method::FunctionName;
use crate::utils::misc::dart_maybe_implements_exception;
use crate::{ir::*, Opts};
use convert_case::{Case, Casing};

type_dart_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeDartGeneratorTrait for TypeStructRefGenerator<'_> {}
