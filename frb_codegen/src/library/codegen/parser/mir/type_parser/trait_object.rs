use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;
use syn::TypeTraitObject;
use crate::utils::syn_utils::ty_to_string;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_trait_object(
        &mut self,
        type_trait_object: &TypeTraitObject,
    ) -> anyhow::Result<MirType> {
        if let [syn::TypeParamBound::Trait(trait_bound)] = &type_trait_object.bounds {
        }
        todo!()
    }
}
