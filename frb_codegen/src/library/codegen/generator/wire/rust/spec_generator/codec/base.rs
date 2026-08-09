use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::get_interest_types_for_codec;
use crate::codegen::generator::codec::structs::EncodeOrDecode;
use crate::codegen::generator::codec::structs::{BaseCodecEntrypointTrait, CodecMode};
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::entrypoint::CstWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::entrypoint::DcoWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::pde::entrypoint::PdeWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::codec::sse::entrypoint::SseWireRustCodecEntrypoint;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFuncParam;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::codegen_codec_structs;
use serde::Serialize;
use std::collections::HashMap;
use strum::IntoEnumIterator;

codegen_codec_structs!(Rust);

impl WireRustCodecOutputSpec {
    pub(crate) fn deduplicate_coherent_impls(mut self) -> Self {
        let mut common_impls = HashMap::new();
        self.inner.common = deduplicate_output_codes(self.inner.common, &mut common_impls);

        let mut io_impls = common_impls.clone();
        self.inner.io = deduplicate_output_codes(self.inner.io, &mut io_impls);

        let mut web_impls = common_impls;
        self.inner.web = deduplicate_output_codes(self.inner.web, &mut web_impls);

        self
    }
}

fn deduplicate_output_codes(
    codes: Vec<WireRustOutputCode>,
    coherent_impls: &mut HashMap<String, String>,
) -> Vec<WireRustOutputCode> {
    codes
        .into_iter()
        .filter(|code| match &code.coherence_key {
            Some(key) => match coherent_impls.get(key) {
                Some(existing_body) => existing_body != &code.body,
                None => {
                    coherent_impls.insert(key.clone(), code.body.clone());
                    true
                }
            },
            None => true,
        })
        .collect()
}

pub(crate) trait WireRustCodecEntrypointTrait<'a>:
    BaseCodecEntrypointTrait<WireRustGeneratorContext<'a>, WireRustCodecOutputSpec>
{
    fn generate_func_params(
        &self,
        func: &MirFunc,
        context: WireRustGeneratorContext,
    ) -> Acc<Vec<ExternFuncParam>>;

    fn generate_func_call_decode(
        &self,
        func: &MirFunc,
        context: WireRustGeneratorContext,
    ) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps one coherent impl per target while preserving unkeyed output.
    #[test]
    fn deduplicate_coherent_impls_respects_common_output() {
        let duplicate = WireRustOutputCode {
            body: "duplicate".to_owned(),
            coherence_key: Some("impl Trait for Type".to_owned()),
            ..Default::default()
        };
        let unique = WireRustOutputCode {
            body: "unique".to_owned(),
            ..Default::default()
        };
        let conflicting = WireRustOutputCode {
            body: "conflicting".to_owned(),
            coherence_key: duplicate.coherence_key.clone(),
            ..Default::default()
        };
        let output = WireRustCodecOutputSpec {
            inner: Acc {
                common: vec![duplicate.clone()],
                io: vec![duplicate.clone(), conflicting.clone(), unique.clone()],
                web: vec![duplicate, conflicting, unique],
            },
        }
        .deduplicate_coherent_impls();

        assert_eq!(output.inner.common.len(), 1);
        assert_eq!(output.inner.io.len(), 2);
        assert_eq!(output.inner.web.len(), 2);
        assert_eq!(output.inner.io[0].body, "conflicting");
        assert_eq!(output.inner.web[0].body, "conflicting");
        assert_eq!(output.inner.io[1].body, "unique");
        assert_eq!(output.inner.web[1].body, "unique");
    }
}
