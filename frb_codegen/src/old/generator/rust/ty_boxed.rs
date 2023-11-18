use super::NO_PARAMS;
use crate::generator::rust::ty::*;
use crate::generator::rust::{generate_import, ExternFuncCollector};
use crate::ir::*;
use crate::target::Acc;
use crate::target::Target::*;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

type_rust_generator_struct!(TypeBoxedGenerator, IrTypeBoxed);

impl TypeRustGeneratorTrait for TypeBoxedGenerator<'_> {}
