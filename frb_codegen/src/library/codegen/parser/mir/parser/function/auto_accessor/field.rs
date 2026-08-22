use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncAccessorMode, MirFuncArgMode, MirFuncImplMode, MirFuncInput, MirFuncMode,
    MirFuncOutput, MirFuncOwnerInfo, MirFuncOwnerInfoMethod, MirFuncOwnerInfoMethodMode,
    OwnershipMode,
};
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::internal_config::ParserMirInternalConfig;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function::auto_accessor::MirFuncAndSanityCheckInfo;
use crate::codegen::parser::mir::parser::function::real::argument::merge_ownership_into_ty;
use crate::codegen::parser::mir::parser::function::real::{
    compute_codec_mode_pack, parse_effective_function_name_of_method,
};
use crate::codegen::parser::mir::parser::function::ui_related::UI_MUTATION_FUNCTION_RUST_AOP_AFTER;
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use crate::codegen::parser::mir::sanity_checker::auto_accessor_checker;
use crate::utils::namespace::NamespacedName;
use sha1::{Digest, Sha1};

#[allow(clippy::too_many_arguments)]
pub(super) fn parse_auto_accessor_of_field(
    config: &ParserMirInternalConfig,
    struct_name: &NamespacedName,
    field: &MirField,
    accessor_mode: MirFuncAccessorMode,
    ty_direct_parse: &MirType,
    type_parser: &mut TypeParser,
    context: &TypeParserParsingContext,
    ty_struct: &MirStruct,
) -> anyhow::Result<MirFuncAndSanityCheckInfo> {
    let rust_method_name = format!(
        "auto_accessor_{}_{}",
        accessor_mode.verb_str(),
        field.name.rust_style(true)
    );

    let owner = MirFuncOwnerInfoMethod {
        owner_ty: ty_direct_parse.to_owned(),
        owner_ty_raw: struct_name.name.to_owned(),
        actual_method_name: rust_method_name,
        actual_method_dart_name: Some(field.name.rust_style(true).to_owned()),
        mode: MirFuncOwnerInfoMethodMode::Instance,
        trait_def: None,
    };

    let mut inputs = vec![compute_self_arg(
        accessor_mode,
        ty_direct_parse,
        type_parser,
        context,
    )?];
    if accessor_mode == MirFuncAccessorMode::Setter {
        inputs.push(MirFuncInput {
            ownership_mode: None,
            inner: create_mir_field(field.ty.clone(), &field.name.rust_style(true)),
            needs_extend_lifetime: false,
        });
    }

    let field_name_rust = field.name.rust_style(true);
    let rust_call_code = match accessor_mode {
        MirFuncAccessorMode::Getter => format!("api_that_guard.{field_name_rust}.clone()"),
        MirFuncAccessorMode::Setter => {
            format!("{{ api_that_guard.{field_name_rust} = api_{field_name_rust}; }}")
        }
    };

    let mir_func = MirFunc {
        namespace: struct_name.namespace.clone(),
        name: MirIdent::new(parse_effective_function_name_of_method(&owner), None),
        id: None,
        inputs,
        output: MirFuncOutput {
            normal: match accessor_mode {
                MirFuncAccessorMode::Getter => field.ty.clone(),
                MirFuncAccessorMode::Setter => MirType::Primitive(MirTypePrimitive::Unit),
            },
            error: None,
        },
        owner: MirFuncOwnerInfo::Method(owner),
        mode: MirFuncMode::Sync,
        stream_dart_await: false,
        rust_async: false,
        initializer: false,
        init_dart_code: None,
        arg_mode: MirFuncArgMode::Named,
        accessor: Some(accessor_mode),
        comments: vec![],
        codec_mode_pack: compute_codec_mode_pack(
            &FrbAttributes::parse(&[]).unwrap(),
            &config.force_codec_mode_pack,
        ),
        rust_call_code: Some(rust_call_code),
        rust_aop_after: (ty_struct.ui_state && accessor_mode == MirFuncAccessorMode::Setter)
            .then(|| UI_MUTATION_FUNCTION_RUST_AOP_AFTER.to_owned()),
        impl_mode: MirFuncImplMode::Normal,
        src_lineno_pseudo: compute_src_lineno_pseudo(struct_name, field),
    };

    Ok(MirFuncAndSanityCheckInfo {
        mir_func,
        sanity_check_hint: auto_accessor_checker::check_field(struct_name, field),
    })
}

fn compute_self_arg(
    accessor_mode: MirFuncAccessorMode,
    ty_direct_parse: &MirType,
    type_parser: &mut TypeParser,
    context: &TypeParserParsingContext,
) -> anyhow::Result<MirFuncInput> {
    let ownership_mode = Some(match accessor_mode {
        MirFuncAccessorMode::Getter => OwnershipMode::Ref,
        MirFuncAccessorMode::Setter => OwnershipMode::RefMut,
    });

    let (ty_interest, ownership_mode) = merge_ownership_into_ty(
        type_parser,
        context,
        ty_direct_parse.to_owned(),
        ownership_mode,
    )?;

    Ok(MirFuncInput {
        ownership_mode,
        inner: create_mir_field(ty_interest, "that"),
        needs_extend_lifetime: false,
    })
}

fn compute_src_lineno_pseudo(struct_name: &NamespacedName, field: &MirField) -> usize {
    let mut hasher = Sha1::new();
    hasher.update(struct_name.rust_style().as_bytes());
    hasher.update(field.name.rust_style(true).as_bytes());
    let digest = hasher.finalize();
    usize::from_le_bytes(digest[..8].try_into().unwrap())
}

fn create_mir_field(ty: MirType, name: &str) -> MirField {
    MirField {
        ty,
        name: MirIdent::new(name.to_owned(), None),
        is_final: true,
        is_rust_public: None,
        comments: vec![],
        default: None,
        settings: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::{compute_src_lineno_pseudo, parse_auto_accessor_of_field};
    use crate::codegen::generator::codec::structs::CodecMode;
    use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::func::{MirFuncAccessorMode, OwnershipMode};
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
    use crate::codegen::ir::mir::ty::structure::MirStruct;
    use crate::codegen::ir::mir::ty::MirType;
    use crate::codegen::parser::mir::internal_config::{
        ParserMirInternalConfig, RustInputNamespacePack,
    };
    use crate::codegen::parser::mir::parser::function::auto_accessor::create_simplified_parsing_context;
    use crate::codegen::parser::mir::parser::function::ui_related::UI_MUTATION_FUNCTION_RUST_AOP_AFTER;
    use crate::codegen::parser::mir::parser::ty::TypeParser;
    use crate::codegen::parser::mir::ParseMode;
    use crate::utils::namespace::{Namespace, NamespacedName};

    /// Derives a stable pseudo line number from the fully qualified field name.
    #[test]
    fn derives_distinct_stable_pseudo_line_numbers() {
        let struct_name = NamespacedName::new(
            Namespace::new(vec!["crate".into(), "api".into()]),
            "Sample".into(),
        );
        let first = field("first");
        let second = field("second");

        assert_eq!(
            compute_src_lineno_pseudo(&struct_name, &first),
            compute_src_lineno_pseudo(&struct_name, &first)
        );
        assert_ne!(
            compute_src_lineno_pseudo(&struct_name, &first),
            compute_src_lineno_pseudo(&struct_name, &second)
        );
    }

    /// Builds getter and ui-state setter functions with their distinct API contracts.
    #[test]
    fn builds_getter_and_ui_state_setter_for_field() -> anyhow::Result<()> {
        let struct_name = NamespacedName::new(
            Namespace::new(vec!["crate".into(), "api".into()]),
            "Sample".into(),
        );
        let config = config();
        let context = create_simplified_parsing_context(
            struct_name.namespace.clone(),
            &config,
            ParseMode::Normal,
        )?;
        let ir_pack = IrEarlyGeneratorPack::default();
        let mut type_parser = TypeParser::new_from_pack(&ir_pack);
        let field = field("value");
        let ty_struct = struct_type(&struct_name, true);
        let owner_ty = MirType::Primitive(MirTypePrimitive::U8);

        let getter = parse_auto_accessor_of_field(
            &config,
            &struct_name,
            &field,
            MirFuncAccessorMode::Getter,
            &owner_ty,
            &mut type_parser,
            &context,
            &ty_struct,
        )?;
        let setter = parse_auto_accessor_of_field(
            &config,
            &struct_name,
            &field,
            MirFuncAccessorMode::Setter,
            &owner_ty,
            &mut type_parser,
            &context,
            &ty_struct,
        )?;

        assert_eq!(getter.mir_func.inputs.len(), 1);
        assert_eq!(
            getter.mir_func.inputs[0].ownership_mode,
            Some(OwnershipMode::Ref)
        );
        assert_eq!(
            getter.mir_func.output.normal,
            MirType::Primitive(MirTypePrimitive::U8)
        );
        assert_eq!(
            getter.mir_func.rust_call_code.as_deref(),
            Some("api_that_guard.value.clone()")
        );
        assert_eq!(getter.mir_func.rust_aop_after, None);
        assert!(getter.sanity_check_hint.is_none());

        assert_eq!(setter.mir_func.inputs.len(), 2);
        assert_eq!(
            setter.mir_func.inputs[0].ownership_mode,
            Some(OwnershipMode::RefMut)
        );
        assert_eq!(
            setter.mir_func.inputs[1].inner.name.rust_style(true),
            "value"
        );
        assert_eq!(
            setter.mir_func.output.normal,
            MirType::Primitive(MirTypePrimitive::Unit)
        );
        assert_eq!(
            setter.mir_func.rust_call_code.as_deref(),
            Some("{ api_that_guard.value = api_value; }")
        );
        assert_eq!(
            setter.mir_func.rust_aop_after.as_deref(),
            Some(UI_MUTATION_FUNCTION_RUST_AOP_AFTER)
        );
        assert!(setter.sanity_check_hint.is_none());
        Ok(())
    }

    fn field(name: &str) -> MirField {
        MirField {
            ty: MirType::Primitive(MirTypePrimitive::U8),
            name: MirIdent::new(name.into(), None),
            is_final: true,
            is_rust_public: Some(true),
            comments: vec![],
            default: None,
            settings: MirFieldSettings::default(),
        }
    }

    fn config() -> ParserMirInternalConfig {
        ParserMirInternalConfig {
            rust_input_namespace_pack: RustInputNamespacePack {
                rust_input_namespace_prefixes: vec![Namespace::new(vec!["crate".into()])],
                rust_output_path_namespace: Namespace::default(),
            },
            force_codec_mode_pack: None,
            default_stream_sink_codec: CodecMode::Dco,
            default_rust_opaque_codec: RustOpaqueCodecMode::Nom,
            stop_on_error: true,
            enable_lifetime: false,
            type_64bit_int: false,
            default_dart_async: true,
        }
    }

    fn struct_type(name: &NamespacedName, ui_state: bool) -> MirStruct {
        MirStruct {
            name: name.clone(),
            wrapper_name: None,
            fields: vec![],
            is_fields_named: true,
            dart_metadata_raw: vec![],
            ignore: false,
            needs_json_serializable: false,
            generate_hash: true,
            generate_eq: true,
            dart_collection_deep_equality: false,
            ui_state,
            comments: vec![],
        }
    }
}
