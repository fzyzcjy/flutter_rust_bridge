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
    let mut ans = Vec::new();
    for ty in [
        info.rust_api_type.rust_api_type(),
        info.cleared_rust_api_type(),
    ] {
        // A type that already has a built-in delegate renders fully qualified
        // (`uuid::Uuid`, `std::net::Ipv4Addr`), while the caller matches against
        // the bare path segment. Without the segment the custom ser/des never
        // matches and the built-in silently wins.
        let last = ty.rsplit("::").next().unwrap_or(&ty).to_owned();
        for candidate in [ty, last] {
            if !ans.contains(&candidate) {
                ans.push(candidate);
            }
        }
    }
    ans
}
