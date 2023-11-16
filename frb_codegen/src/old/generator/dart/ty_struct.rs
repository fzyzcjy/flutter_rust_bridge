use crate::generator::dart::ty::*;
use crate::generator::dart::{dart_comments, dart_metadata, GeneratedApiMethod};
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::method::FunctionName;
use crate::utils::misc::dart_maybe_implements_exception;
use crate::{ir::*, Opts};
use convert_case::{Case, Casing};

type_dart_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeDartGeneratorTrait for TypeStructRefGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: self.context.config.wasm_enabled.then(|| {
                format!(
                    "return [{}];",
                    self.ir
                        .get(self.context.ir_pack)
                        .fields
                        .iter()
                        .map(|field| {
                            format!(
                                "api2wire_{}(raw.{})",
                                field.ty.safe_ident(),
                                field.name.dart_style()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let s = self.ir.get(self.context.ir_pack);
        Some(
            s.fields
                .iter()
                .map(|field| {
                    api_fill_for_field(
                        &field.ty.safe_ident(),
                        &field.name.dart_style(),
                        field.name.rust_style(),
                        field.ty.is_struct(),
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    fn wire2api_body(&self) -> String {
        let src = self.ir.get(self.context.ir_pack);
        let s = self.ir.get(self.context.ir_pack);

        let mut methods = self.context.ir_pack.funcs.iter().filter(|f| {
            let f = FunctionName::deserialize(&f.name);
            f.is_method_for_struct(&src.name) || f.is_static_method_for_struct(&src.name)
        });
        let has_methods = methods.next().is_some();
        let mut inner = s
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                format!(
                    "{}: _wire2api_{}(arr[{}]),",
                    field.name.dart_style(),
                    field.ty.safe_ident(),
                    idx
                )
            })
            .collect::<Vec<_>>();
        if has_methods && self.context.config.use_bridge_in_method {
            inner.insert(0, "bridge: this,".to_string());
        }

        let inner = inner.join("\n");
        let cast = "final arr = raw as List<dynamic>;".to_string();
        let safe_check = format!("if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');", s.fields.len(), s.fields.len());
        format!(
            "{}
                {}
                return {}({});",
            cast, safe_check, s.name, inner,
        )
    }
}

#[inline]
pub(crate) fn api_fill_for_field(
    safe_ident: &str,
    dart_style: &str,
    rust_style: &str,
    is_struct: bool,
) -> String {
    if is_struct {
        format!(
            "_api_fill_to_wire_{}(apiObj.{}, wireObj.{});",
            safe_ident, dart_style, rust_style
        )
    } else {
        format!(
            "wireObj.{} = api2wire_{}(apiObj.{});",
            rust_style, safe_ident, dart_style
        )
    }
}

fn generate_api_method(
    func: &IrFunc,
    ir_struct: &IrStruct,
    dart_api_class_name: String,
    config: &Opts,
) -> GeneratedApiMethod {
    let f = FunctionName::deserialize(&func.name);
    let skip_count = usize::from(!f.is_static_method());
    let mut raw_func_param_list = func
        .inputs
        .iter()
        .skip(skip_count) //skip the first as it's the method 'self'
        .map(|input| {
            format!(
                "{required}{} {} {default}",
                input.ty.dart_api_type(),
                input.name.dart_style(),
                required = input.required_modifier(),
                default = input.field_default(false, Some(config))
            )
        })
        .collect::<Vec<_>>();

    if f.is_static_method() && config.use_bridge_in_method {
        raw_func_param_list.insert(0, format!("required {dart_api_class_name} bridge"));
    }

    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();

    let static_function_name = f.method_name();
    let comments = dart_comments(&func.comments);

    let partial = format!(
        "{} {} {}({{ {} }})",
        if f.is_static_method() { "static" } else { "" },
        func.mode.dart_return_type(&func.output.dart_api_type()),
        if f.is_static_method() {
            if static_function_name == "new" {
                format!("new{}", ir_struct.name)
            } else {
                static_function_name.to_case(Case::Camel)
            }
        } else {
            f.method_name().to_case(Case::Camel)
        },
        full_func_param_list.join(","),
    );

    let signature = partial;

    let mut arg_names = func
        .inputs
        .iter()
        .skip(skip_count) //skip the first as it's the method 'self'
        .map(|input| format!("{}:{},", input.name.dart_style(), input.name.dart_style()))
        .collect::<Vec<_>>();

    let implementation = if f.is_static_method() {
        arg_names.push("hint: hint".to_string());
        let arg_names = arg_names.concat();
        format!(
            "{}.{}({})",
            config.get_dart_api_bridge_name(),
            func.name.clone().to_case(Case::Camel),
            arg_names
        )
    } else {
        let arg_names = arg_names.concat();
        format!(
            "{}.{}({}: this, {})",
            config.get_dart_api_bridge_name(),
            func.name.clone().to_case(Case::Camel),
            func.inputs[0].name.dart_style(),
            arg_names
        )
    };

    GeneratedApiMethod {
        signature,
        implementation,
        comments,
    }
}
