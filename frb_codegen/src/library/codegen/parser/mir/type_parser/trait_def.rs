use crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_trait(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        if let Some(trait_info) = self.inner.src_traits.get(last_segment.0) {
            return Ok(Some(MirType::TraitDef(MirTypeTraitDef {
                name: trait_info.name.clone(),
            })));
        }
        Ok(None)
    }
}
