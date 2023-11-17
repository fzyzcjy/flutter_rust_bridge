use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::dart_api::class::ty::structure_method::generate_api_method;
use crate::codegen::generator::dart_api::class::ty::DartApiGeneratorClassTrait;
use crate::codegen::generator::dart_api::internal_config::GeneratorDartApiInternalConfig;
use crate::codegen::generator::dart_api::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception, generate_dart_metadata,
};
use crate::codegen::ir::func::{
    IrFunc, IrFuncMode, IrFuncOwnerInfo, IrFuncOwnerInfoMethod, IrFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

impl<'a> DartApiGeneratorClassTrait for StructRefDartApiGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let methods = self.context.ir_pack
            .funcs
            .iter()
            .filter(|f| {
                matches!(&f.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod{ struct_name, .. }) if struct_name == &src.name)
            })
            .collect_vec();

        let has_methods = !methods.is_empty();

        let methods = methods
            .iter()
            .map(|func| generate_api_method(func, src, &self.context))
            .collect_vec()
            .concat();

        let extra_argument = if self.context.config.use_bridge_in_method {
            "required this.bridge,".to_string()
        } else {
            "".to_string()
        };

        let field_bridge = if self.context.config.use_bridge_in_method {
            format!("final {} bridge;", self.context.config.dart_api_class_name)
        } else {
            String::new()
        };

        let private_constructor = if has_methods {
            format!("const {}._();", self.ir.ident.0)
        } else {
            "".to_owned()
        };

        Some(if src.using_freezed() {
            self.generate_mode_freezed(
                src,
                &comments,
                &metadata,
                has_methods,
                &methods,
                private_constructor,
            )
        } else {
            self.generate_mode_non_freezed(
                src,
                comments,
                metadata,
                has_methods,
                methods,
                extra_argument,
                field_bridge,
            )
        })
    }
}

impl<'a> StructRefDartApiGenerator<'a> {
    fn generate_mode_freezed(
        &self,
        src: &IrStruct,
        comments: &String,
        metadata: &String,
        has_methods: bool,
        methods: &String,
        private_constructor: String,
    ) -> String {
        let mut constructor_params = src
            .fields
            .iter()
            .map(|field| {
                let r#default =
                    generate_field_default(field, true, self.context.config.dart_enums_style);
                format!(
                    "{default} {} {} {},",
                    generate_field_required_modifier(field),
                    DartApiGenerator::new(field.ty.clone(), self.context.clone()).dart_api_type(),
                    field.name.dart_style()
                )
            })
            .collect_vec();
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
            methods,
            comments = comments,
            private_constructor = private_constructor,
            meta = metadata,
            Name = self.ir.ident.0,
            implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception),
        )
    }

    fn generate_mode_non_freezed(
        &self,
        src: &IrStruct,
        comments: String,
        metadata: String,
        has_methods: bool,
        methods: String,
        extra_argument: String,
        field_bridge: String,
    ) -> String {
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
            .collect_vec();
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
                    default =
                        generate_field_default(f, false, self.context.config.dart_enums_style)
                )
            })
            .collect_vec();
        if has_methods {
            constructor_params.insert(0, extra_argument);
        }

        let (left, right) = if constructor_params.is_empty() {
            ("", "")
        } else {
            ("{", "}")
        };
        let constructor_params = constructor_params.join("");
        let const_capable = src.fields.iter().all(|field| field.is_final);

        format!(
            "{comments}{meta}class {Name} {implements_exception} {{
                    {}

                    {maybe_const}{Name}({}{}{});

                {}
            }}",
            field_declarations,
            left,
            constructor_params,
            right,
            methods,
            comments = comments,
            meta = metadata,
            Name = self.ir.ident.0,
            maybe_const = if const_capable { "const " } else { "" },
            implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception),
        )
    }
}
