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

impl<'a> StructRefDartApiGenerator<'a> {
    pub(super) fn generate_mode_non_freezed(
        &self,
        src: &IrStruct,
        comments: &str,
        metadata: &str,
        has_methods: bool,
        methods: &str,
        extra_argument: &str,
        field_bridge: &str,
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
