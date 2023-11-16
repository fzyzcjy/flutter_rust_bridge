use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::DartApiGeneratorClassTrait;
use crate::codegen::generator::dart_api::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception, generate_dart_metadata,
    generate_field_required_modifier,
};
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;

impl<'a> DartApiGeneratorClassTrait for StructRefDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let ir_pack = self.context.ir_pack;
        let methods = ir_pack
            .funcs
            .iter()
            .filter(|f| {
                let f = FunctionName::deserialize(&f.name);
                f.is_method_for_struct(&src.name) || f.is_static_method_for_struct(&src.name)
            })
            .collect::<Vec<_>>();

        let has_methods = !methods.is_empty();
        let methods = methods
            .iter()
            .map(|func| {
                generate_api_method(
                    func,
                    src,
                    self.context.config.dart_api_class_name.to_string(),
                    self.context.config,
                )
            })
            .collect::<Vec<_>>();

        let methods_string = methods
            .iter()
            .map(|g| {
                format!(
                    "{}{}=>{};\n\n",
                    g.comments,
                    g.signature.clone(),
                    g.implementation.clone()
                )
            })
            .collect::<Vec<_>>()
            .concat();
        let extra_argument = if self.context.config.use_bridge_in_method {
            "required this.bridge,".to_string()
        } else {
            "".to_string()
        };
        let field_bridge = if self.context.config.use_bridge_in_method {
            format!("final {} bridge;", self.context.config.dart_api_class_name,)
        } else {
            String::new()
        };
        let private_constructor = if has_methods {
            format!("const {}._();", self.ir.ident.0)
        } else {
            "".to_owned()
        };
        Some(if src.using_freezed() {
            let mut constructor_params = src
                .fields
                .iter()
                .map(|field| {
                    let r#default = field.field_default(true, Some(self.context.config));
                    format!(
                        "{default} {} {} {},",
                        generate_field_required_modifier(field),
                        DartApiGenerator::new(field.ty.clone(), self.context.clone())
                            .dart_api_type(),
                        field.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods && self.context.config.use_bridge_in_method {
                constructor_params.insert(
                    0,
                    format!(
                        "required {} bridge,",
                        self.context.config.dart_api_class_name
                    ),
                );
            }
            let constructor_params = constructor_params.join("");

            format!(
                "{comments}{meta}class {Name} with _${Name} {implements_exception} {{
                    {private_constructor}
                    const factory {Name}({{{}}}) = _{Name};
                    {}
                }}",
                constructor_params,
                methods_string,
                comments = comments,
                private_constructor = private_constructor,
                meta = metadata,
                Name = self.ir.ident.0,
                implements_exception =
                    generate_dart_maybe_implements_exception(self.ir.is_exception),
            )
        } else {
            let mut field_declarations = src
                .fields
                .iter()
                .map(|f| {
                    let comments = generate_dart_comments(&f.comments);
                    format!(
                        "{}{} {} {};",
                        comments,
                        if f.is_final { "final" } else { "" },
                        DartApiGenerator::new(f.ty.clone(), self.context.clone()).dart_api_type(),
                        f.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            if has_methods {
                field_declarations.insert(0, field_bridge);
            }
            let field_declarations = field_declarations.join("\n");
            let mut constructor_params = src
                .fields
                .iter()
                .map(|f| {
                    format!(
                        "{required}this.{} {default},",
                        f.name.dart_style(),
                        required = generate_field_required_modifier(f),
                        default = f.field_default(false, Some(self.context.config))
                    )
                })
                .collect::<Vec<_>>();
            if has_methods {
                constructor_params.insert(0, extra_argument);
            }

            let (left, right) = if constructor_params.is_empty() {
                ("", "")
            } else {
                ("{", "}")
            };
            let constructor_params = constructor_params.join("");
            let const_capable = if src.const_capable() { "const " } else { "" };

            format!(
                "{comments}{meta}class {Name} {implements_exception} {{
                    {}
        
                    {const}{Name}({}{}{});
        
                {}
            }}",
                field_declarations,
                left,
                constructor_params,
                right,
                methods_string,
                comments = comments,
                meta = metadata,
                Name = self.ir.ident.0,
                const = const_capable,
                implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception),
            )
        })
    }
}
