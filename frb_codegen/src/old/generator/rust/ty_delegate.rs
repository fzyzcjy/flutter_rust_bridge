use crate::generator::rust::ty::*;
use crate::generator::rust::{
    generate_list_allocate_func, ExternFuncCollector, TypeGeneralListGenerator,
};
use crate::ir::*;
use crate::target::{Acc, Target};
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

use super::generate_impl_into_into_dart;

type_rust_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

impl TypeRustGeneratorTrait for TypeDelegateGenerator<'_> {}
