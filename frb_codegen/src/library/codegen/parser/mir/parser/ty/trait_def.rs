use crate::codegen::ir::mir::ty::trait_def::MirTypeTraitDef;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserWithContext};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_trait(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        Ok(parse_type_trait(last_segment.0, self.inner).map(MirType::TraitDef))
    }
}

pub(crate) fn parse_type_trait(name: &str, type_parser: &TypeParser) -> Option<MirTypeTraitDef> {
    (type_parser.src_traits.get(name)).map(|trait_info| MirTypeTraitDef {
        name: trait_info.name.clone(),
    })
}
