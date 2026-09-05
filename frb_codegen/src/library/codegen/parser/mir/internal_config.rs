use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::utils::namespace::Namespace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserMirInternalConfig {
    pub rust_input_namespace_pack: RustInputNamespacePack,
    pub force_codec_mode_pack: Option<CodecModePack>,
    pub default_stream_sink_codec: CodecMode,
    pub default_rust_opaque_codec: RustOpaqueCodecMode,
    pub stop_on_error: bool,
    pub enable_lifetime: bool,
    pub type_64bit_int: bool,
    pub default_dart_async: bool,
}

// TODO rename - this is no longer an "input-namespace"-only pack
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RustInputNamespacePack {
    pub rust_input_namespace_prefixes: Vec<Namespace>,
    pub rust_output_path_namespace: Namespace,
}

impl RustInputNamespacePack {
    pub fn is_interest(&self, namespace: &Namespace) -> bool {
        (self.rust_input_namespace_prefixes.iter()).any(|prefix| prefix.is_prefix_of(namespace))
    }
}

#[cfg(test)]
mod tests {
    use super::RustInputNamespacePack;
    use crate::utils::namespace::Namespace;

    /// Matches configured namespaces and their descendants only.
    #[test]
    fn recognizes_interesting_namespace_prefixes() {
        let pack = RustInputNamespacePack {
            rust_input_namespace_prefixes: vec![Namespace::new(vec!["crate".into(), "api".into()])],
            rust_output_path_namespace: Namespace::default(),
        };

        assert!(pack.is_interest(&Namespace::new(vec!["crate".into(), "api".into()])));
        assert!(pack.is_interest(&Namespace::new(vec![
            "crate".into(),
            "api".into(),
            "nested".into(),
        ])));
        assert!(!pack.is_interest(&Namespace::new(vec!["crate".into(), "other".into()])));
    }
}
