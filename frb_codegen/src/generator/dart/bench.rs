use crate::target::Acc;

use super::GeneratedBenchFunc;

#[allow(dead_code)]
pub(crate) fn push_benchmark_funcs(
    lines: &mut Acc<Vec<String>>,
    dart_bench_funcs: &[GeneratedBenchFunc],
) {
    lines.io.push(
        dart_bench_funcs
            .iter()
            .map(|x| {
                format!(
                    "{} {{
              {}
            }}",
                    x.io.signature, x.io.implementation
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
    );
    lines.wasm.push(
        dart_bench_funcs
            .iter()
            .map(|x| {
                format!(
                    "{} {{
              {}
            }}",
                    x.wasm.signature, x.wasm.implementation
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
    );
    lines.common.push(
        dart_bench_funcs
            .iter()
            .map(|x| {
                format!(
                    "{} {{
                      {} {{
                        {}
                      }}
                    }}",
                    x.common.extend, x.common.signature, x.common.implementation
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n"),
    );
}
