use crate::ir::IrField;
use crate::ir::IrFunc;
use crate::ir::IrTypeTrait;
use crate::target::Acc;
use crate::Opts;

use super::GeneratedBenchFunc;

impl IrFunc {
    pub(crate) fn output_dart_api_type(&self) -> String {
        self.output.dart_api_type()
    }
    pub(crate) fn name_dart_format(&self) -> String {
        use convert_case::{Case, Casing};
        self.name.clone().to_case(Case::Camel)
    }
    fn as_suffix(&self) -> String {
        use convert_case::{Case, Casing};
        self.name.clone().to_case(Case::UpperCamel)
    }
    /// exposed wrapping bench function
    pub(crate) fn bench_fn_name(&self) -> String {
        format!("bench{}", self.as_suffix())
    }
    pub(crate) fn bench_wire_fn_name(&self) -> String {
        format!("wire{}", self.as_suffix())
    }
    pub(crate) fn bench_extension_name(&self) -> String {
        format!("Bench{}Extension", self.as_suffix())
    }
    pub(crate) fn bench_extesion_on(&self, implementation: &str) -> String {
        format!(
            "extension {} on {implementation}",
            self.bench_extension_name()
        )
    }
    /// list of inputs in method signature
    pub(crate) fn as_typed_dart_inputs(&self) -> String {
        self.inputs
            .iter()
            .map(generate_bench_func_input)
            .collect::<Vec<_>>()
            .join(",")
    }
    /// list of params on method call
    pub(crate) fn as_named_dart_params(&self) -> String {
        self.inputs
            .iter()
            .map(comma_separated)
            .collect::<Vec<_>>()
            .join(",")
    }
    pub(crate) fn as_universal_bench_fn_signature(&self) -> String {
        if self.inputs.is_empty() {
            return format!(
                "Future<{}> {}({{String? timelineTaskName}}) async",
                self.output_dart_api_type(),
                self.bench_fn_name()
            );
        }
        format!(
            "Future<{}> {}({{{}, String? timelineTaskName}}) async",
            self.output_dart_api_type(),
            self.bench_fn_name(),
            self.as_typed_dart_inputs()
        )
    }
    pub(crate) fn as_universal_bench_fn_body(&self) -> String {
        if self.inputs.is_empty() {
            return format!(
                "return {}(this,timelineTaskName);",
                self.bench_wire_fn_name()
            );
        }
        format!(
            "return {}(this,timelineTaskName,{});",
            self.bench_wire_fn_name(),
            self.as_named_dart_params()
        )
    }
    pub(crate) fn as_io_signature(&self, config: &Opts) -> String {
        if self.inputs.is_empty() {
            return format!(
                "Future<{}> {}({} bridge, String? timelineTaskName) async",
                self.output_dart_api_type(),
                self.bench_wire_fn_name(),
                config.dart_api_impl_class_name()
            );
        }
        format!(
            "Future<{}> {}({} bridge, String? timelineTaskName, {{{}}}) async",
            self.output_dart_api_type(),
            self.bench_wire_fn_name(),
            config.dart_api_impl_class_name(),
            self.as_typed_dart_inputs()
        )
    }
    /// io specific method body
    pub(crate) fn as_io_body(&self) -> String {
        if self.output_dart_api_type() != "void" {
            return format!(
                "
                final task = TimelineTask();
                if (timelineTaskName != null && timelineTaskName.isNotEmpty) {{
                  task.start('Bench $timelineTaskName');
                }} else {{
                  task.start('Bench {}');
                }}
                return bridge.{}({})
                .then((value) => value)
                .whenComplete(() {{
                  task.finish();
                }});",
                self.name,
                self.name_dart_format(),
                self.as_named_dart_params()
            );
        }
        format!(
            "
            final task = TimelineTask();
            if (timelineTaskName != null && timelineTaskName.isNotEmpty) {{
              task.start('Bench $timelineTaskName');
            }} else {{
              task.start('Bench {}');
            }}
            bridge.{}().whenComplete(() {{
              task.finish();
            }});",
            self.name,
            self.name_dart_format(),
        )
    }
    pub(crate) fn as_wasm_signature(&self, config: &Opts) -> String {
        if self.inputs.is_empty() {
            return format!(
                "Future<{}> {}({} bridge, String? timelineTaskName) async",
                self.output_dart_api_type(),
                self.wire_func_name(),
                config.dart_api_impl_class_name(),
            );
        }
        format!(
            "Future<{}> {}({} bridge, String? timelineTaskName, {{{}}}) async",
            self.output_dart_api_type(),
            self.wire_func_name(),
            config.dart_api_impl_class_name(),
            self.as_typed_dart_inputs()
        )
    }
    pub(crate) fn as_wasm_body(&self) -> String {
        if self.output_dart_api_type() != "void" {
            return format!(
                "
                final stopwatch = Stopwatch();
                final int starts = stopwatch.elapsedMicroseconds;
                stopwatch.start();
                return bridge.{}({})
                .then((value) => value)
                .whenComplete(() {{
                  stopwatch.stop();
                  final int ends = stopwatch.elapsedMicroseconds;
                  final int diff = ends - starts;
                  if (timelineTaskName != null && timelineTaskName.isNotEmpty) {{
                    print('Bench [$timelineTaskName] {} executed in $diff microsecond(s)');
                  }} else {{
                    print('Bench {2} executed in $diff microsecond(s)');
                  }}
                }});
                ",
                self.name_dart_format(),
                self.as_named_dart_params(),
                self.name
            );
        }
        format!(
            "
            final stopwatch = Stopwatch();
            final int starts = stopwatch.elapsedMicroseconds;
            stopwatch.start();
            bridge.{}({}).whenComplete(() {{
              stopwatch.stop();
              final int ends = stopwatch.elapsedMicroseconds;
              final int diff = ends - starts;
              if (timelineTaskName != null && timelineTaskName.isNotEmpty) {{
                print('Bench [$timelineTaskName] {} executed in $diff microsecond(s)');
              }} else {{
                print('Bench {2} executed in $diff microsecond(s)');
              }}
            }});",
            self.name_dart_format(),
            self.as_named_dart_params(),
            self.name
        )
    }
}

#[inline]
fn generate_bench_func_input(field: &IrField) -> String {
    use convert_case::{Case, Casing};
    let field_ty = field.ty.dart_api_type();
    let field_name = field.name.to_string().to_case(Case::Camel);
    if field.ty.is_optional() {
        return format!("{field_ty} {field_name}");
    }
    format!("required {field_ty} {field_name}")
}

#[inline]
fn comma_separated(field: &IrField) -> String {
    use convert_case::{Case, Casing};
    format!("{}: {0}", field.name.to_string().to_case(Case::Camel))
}

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
