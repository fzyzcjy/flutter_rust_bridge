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
        let ans = (self.inner.custom_ser_des_infos.iter())
            .find(|info| {
                compute_matcher_types(&*info.rust_api_type).contains(&last_segment.0.to_owned())
            })
            .map(|info| {
                MirType::Delegate(MirTypeDelegate::CustomSerDes(MirTypeDelegateCustomSerDes {
                    info: info.to_owned(),
                }))
            });
        log::info!("hi parse_type_path_data_custom_ser_des last_segment={last_segment:?} ans={ans:?} self.inner.custom_ser_des_infos={:?}", self.inner.custom_ser_des_infos);
        Ok(ans)
    }
}

fn compute_matcher_types(ty: &MirType) -> Vec<String> {
    let mut ans = vec![ty.rust_api_type()];
    if let MirType::RustAutoOpaqueImplicit(ty) = ty {
        ans.push(ty.raw.string.with_original_lifetime().to_owned());
    }
    ans
}
