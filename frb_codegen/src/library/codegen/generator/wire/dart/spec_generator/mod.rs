use crate::codegen::dumper::Dumper;
use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::EncodeOrDecode::{Decode, Encode};
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::{
    WireDartCodecEntrypoint, WireDartCodecOutputSpec,
};
use crate::codegen::generator::wire::dart::spec_generator::dump::generate_dump_info;
use crate::codegen::generator::wire::dart::spec_generator::misc::WireDartOutputSpecMisc;
use crate::codegen::generator::wire::dart::spec_generator::output_code::{
    DartApiImplClassMethod, WireDartOutputCode,
};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::ir::mir::pack::MirPackComputedCache;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::ConfigDumpContent::GeneratorInfo;
use itertools::Itertools;
use serde::Serialize;
use std::path::PathBuf;

pub(crate) mod base;
pub(crate) mod codec;
mod dump;
pub(crate) mod misc;
pub(crate) mod output_code;
pub(super) mod wire_class;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpec {
    pub(super) misc: WireDartOutputSpecMisc,
    pub(super) rust2dart: WireDartCodecOutputSpec,
    pub(super) dart2rust: WireDartCodecOutputSpec,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
    rust_extern_funcs: &[ExternFunc],
    rust_content_hash: i32,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<WireDartOutputSpec> {
    let cache = MirPackComputedCache::compute(context.mir_pack);

    (dumper.with_content(GeneratorInfo))
        .dump("wire_dart.json", &generate_dump_info(&cache, context))?;

    Ok(WireDartOutputSpec {
        misc: misc::generate(
            context,
            &cache,
            c_file_content,
            api_dart_actual_output_paths,
            rust_extern_funcs,
            rust_content_hash,
            progress_bar_pack,
        )?,
        rust2dart: auto_add_base_class_abstract_method(WireDartCodecEntrypoint::generate_all(
            context, &cache, Decode,
        )),
        dart2rust: auto_add_base_class_abstract_method(WireDartCodecEntrypoint::generate_all(
            context, &cache, Encode,
        )),
    })
}

fn auto_add_base_class_abstract_method(raw: WireDartCodecOutputSpec) -> WireDartCodecOutputSpec {
    let Acc {
        common,
        mut io,
        mut web,
    } = raw.inner;

    let extra_abstract_methods = (common.iter())
        .flat_map(|x| x.api_impl_class_methods.clone())
        .map(|method| DartApiImplClassMethod {
            signature: method.signature,
            body: None,
        })
        .collect_vec();
    let extra_item = WireDartOutputCode {
        api_impl_class_methods: extra_abstract_methods,
        ..Default::default()
    };

    io.push(extra_item.clone());
    web.push(extra_item);

    WireDartCodecOutputSpec {
        inner: Acc { common, io, web },
    }
}
