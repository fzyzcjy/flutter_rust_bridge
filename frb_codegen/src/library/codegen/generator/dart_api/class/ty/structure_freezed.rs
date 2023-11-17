use crate::codegen::generator::dart_api::base::*;
use crate::codegen::generator::dart_api::class::field::{
    generate_field_default, generate_field_required_modifier,
};

use crate::codegen::generator::dart_api::misc::generate_dart_maybe_implements_exception;

use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::dart_api::decl::DartApiGeneratorDeclTrait;

use itertools::Itertools;

impl<'a> StructRefDartApiGenerator<'a> {
    pub(super) fn generate_mode_freezed(
        &self,
        src: &IrStruct,
        comments: &str,
        metadata: &str,
        methods: &[String],
    ) -> String {
        let private_constructor = if !methods.is_empty() {
            format!("const {}._();", self.ir.ident.0)
        } else {
            "".to_owned()
        };

        let constructor_params =
            self.generate_mode_freezed_constructor_params(src, !methods.is_empty());
        let name_str = &self.ir.ident.0;
        let implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception);
        let methods_str = methods.join("\n");

        format!(
            "{comments}{metadata}class {name_str} with _${name_str} {implements_exception} {{
                {private_constructor}
                const factory {name_str}({{{constructor_params}}}) = _{name_str};
                {methods_str}
            }}",
        )
    }

    fn generate_mode_freezed_constructor_params(
        &self,
        src: &IrStruct,
        has_methods: bool,
    ) -> String {
        let mut ans = src
            .fields
            .iter()
            .map(|field| {
                let r#default =
                    generate_field_default(field, true, self.context.config.dart_enums_style);
                let required_modifier = generate_field_required_modifier(field);
                let type_str =
                    DartApiGenerator::new(field.ty.clone(), self.context.clone()).dart_api_type();
                let name = field.name.dart_style();
                format!("{default} {required_modifier} {type_str} {name},")
            })
            .collect_vec();

        if has_methods && self.context.config.use_bridge_in_method {
            ans.insert(
                0,
                format!(
                    "required {} bridge,",
                    self.context.config.dart_api_class_name
                ),
            );
        }

        ans.join("")
    }
}
