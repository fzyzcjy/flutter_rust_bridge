use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::mir::type_parser::TypeParserWithContext;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_trait(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        TODO
    }
}
