use crate::utils::console::SimpleProgressBar;

pub(crate) struct GeneratorProgressBarPack {
    pub parse: SimpleProgressBar,
    pub parse_cargo_expand: SimpleProgressBar,
    pub parse_source_graph: SimpleProgressBar,
    pub generate: SimpleProgressBar,
    pub generate_cbindgen: SimpleProgressBar,
    pub generate_ffigen: SimpleProgressBar,
    pub polish: SimpleProgressBar,
    pub polish_dart_build_runner: SimpleProgressBar,
    pub polish_dart_formatter: SimpleProgressBar,
    pub polish_rust_formatter: SimpleProgressBar,
}
