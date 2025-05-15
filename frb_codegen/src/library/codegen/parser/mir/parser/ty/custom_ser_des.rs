use crate::codegen::ir::mir::custom_ser_des::MirCustomSerDes;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateCustomSerDes};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_type_path_data_custom_ser_des(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        // use HashMap etc later if too slow; here we use filter to remain flexibility of filtering strategy
        Ok((self.inner.custom_ser_des_infos.iter())
            .find(|info| compute_matcher_types(info).contains(&last_segment.0.to_owned()))
            .map(|info| {
                MirType::Delegate(MirTypeDelegate::CustomSerDes(MirTypeDelegateCustomSerDes {
                    info: info.to_owned(),
                }))
            }))
    }
}

fn compute_matcher_types(info: &MirCustomSerDes) -> Vec<String> {
    vec![
        info.rust_api_type.rust_api_type(),
        info.cleared_rust_api_type(),
    ]
}
