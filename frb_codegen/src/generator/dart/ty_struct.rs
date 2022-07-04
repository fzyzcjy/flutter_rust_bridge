use convert_case::{Case, Casing};

use super::GeneratedApiFunc;
use crate::generator::dart::TypeDartGenerator::Primitive;
use crate::generator::dart::{dart_comments, dart_metadata, GeneratedApiMethod};
use crate::generator::dart::{is_method, ty::*};
use crate::ir::*;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeDartGeneratorTrait for TypeStructRefGenerator<'_> {
    fn api2wire_body(&self) -> Option<String> {
        None
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let s = self.ir.get(self.context.ir_file);
        Some(
            s.fields
                .iter()
                .map(|field| {
                    format!(
                        "wireObj.{} = _api2wire_{}(apiObj.{});",
                        field.name.rust_style(),
                        field.ty.safe_ident(),
                        field.name.dart_style()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    fn wire2api_body(&self) -> String {
        let s = self.ir.get(self.context.ir_file);
        let inner = s
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
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "final arr = raw as List<dynamic>;
                if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');
                return {}({});",
            s.fields.len(),
            s.fields.len(),
            s.name, inner,
        )
    }

    fn set_dart_api_class_name(&mut self, s: &str) {
        self.dart_api_class_name = Some(s.to_string())
    }

    fn structs(&self) -> String {
        let src = self.ir.get(self.context.ir_file);
        println!("src struct: {:?}", src);
        let comments = dart_comments(&src.comments);
        let metadata = dart_metadata(&src.dart_metadata);

        let ir_file = self.context.ir_file;
        let methods = ir_file
            .funcs
            .iter()
            .filter(|f| is_method(f, src.name.clone()))
            .collect::<Vec<_>>();
        println!("methods size: {}, content: {:?}", methods.len(), methods);
        let has_methods = !methods.is_empty();
        let methods = methods
            .iter()
            .map(|func| generate_api_method(func, src))
            .collect::<Vec<_>>();
        println!("actual methods: {:?}", methods);
        let methods_string = methods
            .iter()
            .map(|g| format!("{}=>{};", g.signature.clone(), g.implementation.clone()))
            .collect::<Vec<_>>()
            .concat();
        let bridge_requirement = "required this.bridge,".to_string();
        let field_bridge = format!(
            "final {} bridge;",
            self.dart_api_class_name.as_ref().unwrap()
        );
        if src.using_freezed() {
            let mut constructor_params = src
                .fields
                .iter()
                .map(|f| {
                    format!(
                        "{} {} {},",
                        f.ty.dart_required_modifier(),
                        f.ty.dart_api_type(),
                        f.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods {
                constructor_params.insert(0, bridge_requirement);
            }
            let constructor_params = constructor_params.join("");

            format!(
                "{}{}class {} with _${} {{
                const factory {}({{{}}}) = _{};
                {}
            }}",
                comments,
                metadata,
                self.ir.name,
                self.ir.name,
                self.ir.name,
                constructor_params,
                self.ir.name,
                methods_string
            )
        } else {
            let mut field_declarations = src
                .fields
                .iter()
                .map(|f| {
                    let comments = dart_comments(&f.comments);
                    format!(
                        "{}{} {} {};",
                        comments,
                        if f.is_final { "final" } else { "" },
                        f.ty.dart_api_type(),
                        f.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods {
                field_declarations.insert(0, field_bridge);
            }
            let field_declarations = field_declarations.join("\n");
            println!("field_declarations: {}", field_declarations);
            let mut constructor_params = src
                .fields
                .iter()
                .map(|f| {
                    format!(
                        "{}this.{},",
                        f.ty.dart_required_modifier(),
                        f.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods {
                constructor_params.insert(0, bridge_requirement);
            }
            println!("constructor_params: {:?}", constructor_params);
            let constructor_params = constructor_params.join("");

            format!(
                "{}{}class {} {{
                {}

                {}({{{}}});
                {}
            }}",
                comments,
                metadata,
                self.ir.name,
                field_declarations,
                self.ir.name,
                constructor_params,
                methods_string
            )
        }
    }
}

fn generate_api_method(func: &IrFunc, ir_struct: &IrStruct) -> GeneratedApiMethod {
    println!(
        "generate_api_method for {:?} with ir_struct: {:?}",
        func, ir_struct
    );
    let raw_func_param_list = func
        .inputs
        .iter()
        .skip(1) //skip the first as it's the method 'self'
        .map(|input| {
            format!(
                "{}{} {}",
                input.ty.dart_required_modifier(),
                input.ty.dart_api_type(),
                input.name.dart_style()
            )
        })
        .collect::<Vec<_>>();
    println!("raw_func_param_list: {:?}", raw_func_param_list);
    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();
    /*
    let wire_param_list = [
        if func.mode.has_port_argument() {
            vec!["port_".to_string()]
        } else {
            vec![]
        },
        func.inputs
            .iter()
            .map(|input| {
                // edge case: ffigen performs its own bool-to-int conversions
                if let IrType::Primitive(IrTypePrimitive::Bool) = input.ty {
                    input.name.dart_style()
                } else {
                    format!(
                        "_api2wire_{}({})",
                        &input.ty.safe_ident(),
                        &input.name.dart_style()
                    )
                }
            })
            .collect::<Vec<_>>(),
    ]
    .concat();
    println!("wire_param_list: {:?}", wire_param_list);
    */
    let partial = format!(
        "{} {}({{ {} }})",
        func.mode.dart_return_type(&func.output.dart_api_type()),
        func.name.replace("__method", "").to_case(Case::Camel),
        full_func_param_list.join(","),
    );
    /*
    println!("partial: {}", partial);
    let execute_func_name = match func.mode {
        IrFuncMode::Normal => "executeNormal",
        IrFuncMode::Sync => "executeSync",
        IrFuncMode::Stream => "executeStream",
    };
    */
    //let const_meta_field_name = format!("k{}ConstMeta", func.name.replace("__method", "").to_case(Case::Pascal));

    let signature = format!("{}", partial);

    let comments = dart_comments(&func.comments);

    /*
    let task_common_args = format!(
        "
        constMeta: {},
        argValues: [{}],
        hint: hint,
        ",
        const_meta_field_name,
        func.inputs
            .iter()
            .map(|input| input.name.dart_style())
            .collect::<Vec<_>>()
            .join(", "),
    );

    let implementation = match func.mode {
        IrFuncMode::Sync => format!(
            "{} => {}(FlutterRustBridgeSyncTask(
            callFfi: () => inner.{}({}),
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            task_common_args,
        ),
        _ => format!(
            "{} => {}(FlutterRustBridgeTask(
            callFfi: (port_) => inner.{}({}),
            parseSuccessData: _wire2api_{},
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            func.output.safe_ident(),
            task_common_args,
        ),
    };

    let companion_field_signature = format!(
        "FlutterRustBridgeTaskConstMeta get {};",
        const_meta_field_name,
    );

    let companion_field_implementation = format!(
        "
        FlutterRustBridgeTaskConstMeta get {} => const FlutterRustBridgeTaskConstMeta(
            debugName: \"{}\",
            argNames: [{}],
        );
        ",
        const_meta_field_name,
        func.name,
        func.inputs
            .iter()
            .map(|input| format!("\"{}\"", input.name.dart_style()))
            .collect::<Vec<_>>()
            .join(", "),
    );
    */
    let arg_names = func
        .inputs
        .iter()
        .skip(1) //skip the first as it's the method 'self'
        .map(|input| format!("{}:{},", input.name.dart_style(), input.name.dart_style()))
        .collect::<Vec<_>>()
        .concat();
    println!("arg_names: {}", arg_names);
    let implementation = format!(
        "bridge.{}({}: this, {})",
        func.name.clone().to_case(Case::Camel),
        func.inputs[0].name.dart_style(),
        arg_names
    );
    let companion_field_signature = "".to_string();
    let companion_field_implementation = "".to_string();

    GeneratedApiMethod {
        signature,
        implementation,
        comments,
        companion_field_signature,
        companion_field_implementation,
    }
}
