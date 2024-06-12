use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateCustomSerDes};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::ty::unencodable::SplayedSegment;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_custom_ser_des(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        // use HashMap etc later if too slow; here we use filter to remain flexibility of filtering strategy
        Ok((self.inner.custom_ser_des_infos.iter())
            .find(|info| info.rust_api_type.rust_api_type() == last_segment.0)
            .map(|info| {
                MirType::Delegate(MirTypeDelegate::CustomSerDes(MirTypeDelegateCustomSerDes {
                    info: info.to_owned(),
                }))
            }))
    }
}
