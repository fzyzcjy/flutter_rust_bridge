use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

use super::{ExternFuncCollector, NO_PARAMS};

type_rust_generator_struct!(TypeRustOpaqueGenerator, IrTypeRustOpaque);

impl TypeRustGeneratorTrait for TypeRustOpaqueGenerator<'_> {}
