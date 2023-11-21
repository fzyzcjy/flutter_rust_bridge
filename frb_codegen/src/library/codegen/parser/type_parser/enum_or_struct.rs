use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use std::collections::HashMap;
use syn::TypePath;

pub(super) trait EnumOrStructParser<Obj> {
    fn parse(
        &self,
        type_path: &TypePath,
        splayed_segments: &[SplayedSegment],
        last_segment: &SplayedSegment,
    ) {
        if let (name, _) = last_segment {
            if let Some(src_object) = self.src_objects().get(*name) {
                todo!();
            }
        }

        Ok(None)
    }

    fn src_objects(&self) -> &HashMap<String, &Obj>;
}
