use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception,
};
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> StructRefApiDartGenerator<'a> {
    pub(crate) fn generate_mode_non_freezed(
        &self,
        src: &IrStruct,
        comments: &str,
        metadata: &str,
        methods: &[String],
    ) -> String {
        let field_declarations = self.generate_field_declarations(src);
        let constructor_params = self.generate_mode_non_freezed_constructor_params(src);

        let const_capable = src.fields.iter().all(|field| field.is_final);
        let name_str = &self.ir.ident.0.name;
        let maybe_const = if const_capable { "const " } else { "" };
        let implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception);
        let methods_str = methods.join("\n");

        let hashcode = generate_hashcode(&src.fields);
        let equals = generate_equals(&src.fields, name_str);

        format!(
            "{comments}{metadata}class {name_str} {implements_exception} {{
                {field_declarations}

                {maybe_const}{name_str}({constructor_params});

                {methods_str}

                {hashcode}

                {equals}
            }}"
        )
    }

    fn generate_field_declarations(&self, src: &IrStruct) -> String {
        let field_declarations = src
            .fields
            .iter()
            .map(|f| {
                let comments = generate_dart_comments(&f.comments);
                let maybe_final = if f.is_final { "final" } else { "" };
                let type_str = ApiDartGenerator::new(f.ty.clone(), self.context).dart_api_type();
                let name_str = f.name.dart_style();
                format!("{comments}{maybe_final} {type_str} {name_str};")
            })
            .collect_vec();

        field_declarations.join("\n")
    }

    fn generate_mode_non_freezed_constructor_params(&self, src: &IrStruct) -> String {
        let ans = src
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

        let mut ans = ans.join("");

        if !ans.is_empty() {
            ans = format!("{{{ans}}}");
        };

        ans
    }
}

fn generate_hashcode(fields: &[IrField]) -> String {
    let body = if fields.is_empty() {
        "0".to_owned()
    } else {
        fields
            .iter()
            .map(|x| x.name.dart_style())
            .map(|x| format!("{x}.hashCode"))
            .join("^")
    };

    format!(
        "
        @override
        int get hashCode => {body};
        "
    )
}

fn generate_equals(fields: &[IrField], struct_name: &str) -> String {
    let cmp = fields
        .iter()
        .map(|x| x.name.dart_style())
        .map(|x| format!("&& {x} == other.{x}"))
        .join("");

    format!(
        "
        @override
        bool operator ==(Object other) =>
            identical(this, other) ||
            other is {struct_name} &&
                runtimeType == other.runtimeType
                {cmp};
        "
    )
}
