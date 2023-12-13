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

impl GeneratorProgressBarPack {
    pub(crate) fn new() -> Self {
        Self {
            parse: SimpleProgressBar::new("Parse", 0),
            parse_cargo_expand: SimpleProgressBar::new("Run cargo expand", 1),
            parse_source_graph: SimpleProgressBar::new("Parse source graph", 1),
            generate: SimpleProgressBar::new("Generate", 0),
            generate_cbindgen: SimpleProgressBar::new("Run cbindgen", 1),
            generate_ffigen: SimpleProgressBar::new("Run ffigen", 1),
            polish: SimpleProgressBar::new("Polish", 0),
            polish_dart_build_runner: SimpleProgressBar::new("Run Dart build_runner", 1),
            polish_dart_formatter: SimpleProgressBar::new("Run Dart formatter", 1),
            polish_rust_formatter: SimpleProgressBar::new("Run Rust formatter", 1),
        }
    }
}
