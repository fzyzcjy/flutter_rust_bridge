use crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_trait(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        Ok(self.parse_trait(last_segment.0).map(MirType::TraitDef))
    }

    pub(crate) fn parse_trait(&self, name: &str) -> Option<MirTypeTraitDef> {
        (self.inner.src_traits.get(name)).map(|trait_info| MirTypeTraitDef {
            name: trait_info.name.clone(),
        })
    }
}
