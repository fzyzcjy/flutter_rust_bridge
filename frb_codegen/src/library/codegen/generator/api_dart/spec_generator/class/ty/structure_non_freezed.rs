use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::class::method::GeneratedApiMethods;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_maybe_implements_exception,
};
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use itertools::Itertools;

impl StructRefApiDartGenerator<'_> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_mode_non_freezed(
        &self,
        src: &MirStruct,
        comments: &str,
        metadata: &str,
        methods: &GeneratedApiMethods,
        constructor_postfix: &str,
        extra_body: &str,
        class_name: &str,
    ) -> String {
        let field_declarations = self.generate_field_declarations(src);
        let constructor_params = self.generate_mode_non_freezed_constructor_params(src);

        let const_capable = src.fields.iter().all(|field| field.is_final);
        let maybe_const = if const_capable { "const " } else { "" };
        let implements_exception = generate_dart_maybe_implements_exception(self.mir.is_exception);
        let methods_str = &methods.code;
        let dart_collection_deep_equality =
            self.context.config.dart_collection_deep_equality || src.dart_collection_deep_equality;

        let hashcode = if src.generate_hash {
            generate_hashcode(&src.fields, dart_collection_deep_equality)
        } else {
            "".to_owned()
        };
        let equals = if src.generate_eq {
            generate_equals(&src.fields, class_name, dart_collection_deep_equality)
        } else {
            "".to_owned()
        };

        format!(
            "{comments}{metadata}class {class_name} {implements_exception} {{
                {field_declarations}

                {maybe_const}{class_name}{constructor_postfix}({constructor_params});

                {methods_str}
                {extra_body}

                {hashcode}

                {equals}
            }}"
        )
        .lines()
        .map(str::trim_end)
        .join("\n")
    }

    fn generate_field_declarations(&self, src: &MirStruct) -> String {
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

    fn generate_mode_non_freezed_constructor_params(&self, src: &MirStruct) -> String {
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

fn generate_hashcode(fields: &[MirField], dart_collection_deep_equality: bool) -> String {
    let body = if fields.is_empty() {
        "0".to_owned()
    } else {
        fields
            .iter()
            .map(|f| {
                let name = f.name.dart_style();
                if dart_collection_deep_equality && needs_deep_equality(&f.ty) {
                    format!("const DeepCollectionEquality().hash({name})")
                } else {
                    format!("{name}.hashCode")
                }
            })
            .join("^")
    };

    format!(
        "
        @override
        int get hashCode => {body};
        "
    )
}

fn generate_equals(
    fields: &[MirField],
    struct_name: &str,
    dart_collection_deep_equality: bool,
) -> String {
    let cmp = fields
        .iter()
        .map(|f| {
            let name = f.name.dart_style();
            if dart_collection_deep_equality && needs_deep_equality(&f.ty) {
                format!("&& const DeepCollectionEquality().equals({name}, other.{name})")
            } else {
                format!("&& {name} == other.{name}")
            }
        })
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

fn needs_deep_equality(ty: &MirType) -> bool {
    match ty {
        MirType::Boxed(boxed) => needs_deep_equality(&boxed.inner),
        MirType::GeneralList(_) | MirType::PrimitiveList(_) => true,
        MirType::Delegate(delegate) => {
            matches!(
                delegate,
                MirTypeDelegate::Array(_) | MirTypeDelegate::Map(_) | MirTypeDelegate::Set(_)
            )
        }
        MirType::Optional(opt) => needs_deep_equality(&opt.inner),
        _ => false,
    }
}
