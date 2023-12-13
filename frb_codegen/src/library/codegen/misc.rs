use crate::utils::console::SimpleProgressBar;

pub(crate) struct GeneratorProgressBarPack {
    pub primary: SimpleProgressBar,
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
        let ans = Self {
            primary: SimpleProgressBar::new("flutter_rust_bridge_codegen", 0),
            parse: SimpleProgressBar::new("Parse", 1),
            parse_cargo_expand: SimpleProgressBar::new("Run cargo expand", 2),
            parse_source_graph: SimpleProgressBar::new("Parse source graph", 2),
            generate: SimpleProgressBar::new("Generate", 1),
            generate_cbindgen: SimpleProgressBar::new("Run cbindgen", 2),
            generate_ffigen: SimpleProgressBar::new("Run ffigen", 2),
            polish: SimpleProgressBar::new("Polish", 1),
            polish_dart_build_runner: SimpleProgressBar::new("Run Dart build_runner", 2),
            polish_dart_formatter: SimpleProgressBar::new("Run Dart formatter", 2),
            polish_rust_formatter: SimpleProgressBar::new("Run Rust formatter", 2),
        };
        ans.primary.start();
        ans
    }
}
