use crate::codegen::generator::api_dart::base::*;
use crate::codegen::generator::api_dart::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception,
};
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::api_dart::decl::ApiDartGeneratorDeclTrait;
use itertools::Itertools;

impl<'a> StructRefApiDartGenerator<'a> {
    pub(super) fn generate_mode_non_freezed(
        &self,
        src: &IrStruct,
        comments: &str,
        metadata: &str,
        methods: &[String],
    ) -> String {
        let field_declarations = self.generate_field_declarations(src, methods);
        let constructor_params = self.generate_mode_non_freezed_constructor_params(src, methods);

        let const_capable = src.fields.iter().all(|field| field.is_final);
        let name_str = &self.ir.ident.0;
        let maybe_const = if const_capable { "const " } else { "" };
        let implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception);
        let methods_str = methods.join("\n");

        format!(
            "{comments}{metadata}class {name_str} {implements_exception} {{
                {field_declarations}

                {maybe_const}{name_str}({constructor_params});

                {methods_str}
            }}"
        )
    }

    fn generate_field_declarations(&self, src: &IrStruct, methods: &[String]) -> String {
        let mut field_declarations = src
            .fields
            .iter()
            .map(|f| {
                let comments = generate_dart_comments(&f.comments);
                let maybe_final = if f.is_final { "final" } else { "" };
                let type_str =
                    ApiDartGenerator::new(f.ty.clone(), self.context.clone()).dart_api_type();
                let name_str = f.name.dart_style();
                format!("{comments}{maybe_final} {type_str} {name_str};")
            })
            .collect_vec();

        if !methods.is_empty() && self.context.config.use_bridge_in_method {
            field_declarations.insert(
                0,
                format!("final {} bridge;", self.context.config.dart_api_class_name),
            );
        }

        field_declarations.join("\n")
    }

    fn generate_mode_non_freezed_constructor_params(
        &self,
        src: &IrStruct,
        methods: &[String],
    ) -> String {
        let mut ans = src
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

        if !methods.is_empty() && self.context.config.use_bridge_in_method {
            ans.insert(0, "required this.bridge,".to_string());
        }

        let mut ans = ans.join("");

        if !ans.is_empty() {
            ans = format!("{{{ans}}}");
        };

        ans
    }
}
