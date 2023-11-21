use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use syn::TypePath;

pub(super) trait EnumOrStructParser {
    fn parse(
        &self,
        type_path: &TypePath,
        splayed_segments: &[SplayedSegment],
        last_segment: &SplayedSegment,
    ) {
        if let (name, _) = last_segment {
            todo!();
        }

        Ok(None)
    }
}
