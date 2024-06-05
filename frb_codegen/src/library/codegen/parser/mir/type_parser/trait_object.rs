use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::record::MirTypeRecord;
use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent, MirTypeStructRef};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::Primitive;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::NamespacedName;
use anyhow::Result;
use itertools::Itertools;
use syn::{TypeTraitObject, TypeTuple};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_trait_object(
        &mut self,
        type_trait_object: &TypeTraitObject,
    ) -> anyhow::Result<MirType> {
        todo!()
    }
}
