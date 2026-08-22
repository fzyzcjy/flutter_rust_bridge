use crate::codegen::ir::mir::func::{MirFunc, MirFuncInput};
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use itertools::Itertools;

pub(super) fn generate_code_inner_decode(func: &MirFunc, inner: &str) -> String {
    let interest_inputs = (func.inputs.iter())
        .filter(|field| is_interest_field(field))
        .collect_vec();

    let object_static_ref = (interest_inputs.iter())
        .map(|field| {
            generate_illegal_static_reference(&format!(
                "api_{name}",
                name = get_variable_name(field)
            ))
        })
        .join("");

    let guard_static_ref = (interest_inputs.iter())
        .map(|field| {
            let static_ref = generate_illegal_static_reference(&format!(
                "api_{name}_guard",
                name = get_variable_name(field)
            ));
            format!(
                "let api_{name}_guard = Arc::new(api_{name}_guard);
                {static_ref}",
                name = get_variable_name(field)
            )
        })
        .join("");

    format!("{object_static_ref}{inner}{guard_static_ref}")
}

pub(super) fn generate_illegal_static_reference(var_name: &str) -> String {
    format!(
        "let {var_name}_illegal_static_ref = unsafe {{
            flutter_rust_bridge::for_generated::ouroboros_change_lifetime(&{var_name})
        }};"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::codec::structs::{CodecMode, CodecModePack};
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::func::{
        MirFuncArgMode, MirFuncImplMode, MirFuncMode, MirFuncOutput, MirFuncOwnerInfo,
    };
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::delegate::{
        MirTypeDelegate, MirTypeDelegateLifetimeable, MirTypeDelegateRustAutoOpaqueExplicit,
    };
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
        MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
    };
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::utils::namespace::Namespace;

    fn func(needs_extend_lifetime: bool) -> MirFunc {
        MirFunc {
            namespace: Namespace::default(),
            name: MirIdent::new("work".into(), None),
            id: None,
            inputs: vec![MirFuncInput {
                ownership_mode: None,
                inner: MirField {
                    ty: MirType::Primitive(MirTypePrimitive::I32),
                    name: MirIdent::new("handle".into(), None),
                    is_final: false,
                    is_rust_public: None,
                    comments: vec![],
                    default: None,
                    settings: MirFieldSettings::default(),
                },
                needs_extend_lifetime,
            }],
            output: MirFuncOutput {
                normal: MirType::Primitive(MirTypePrimitive::Unit),
                error: None,
            },
            owner: MirFuncOwnerInfo::Function,
            mode: MirFuncMode::Normal,
            stream_dart_await: false,
            rust_async: false,
            initializer: false,
            init_dart_code: None,
            arg_mode: MirFuncArgMode::Named,
            accessor: None,
            comments: vec![],
            codec_mode_pack: CodecModePack {
                dart2rust: CodecMode::Sse,
                rust2dart: CodecMode::Sse,
            },
            rust_call_code: None,
            rust_aop_after: None,
            impl_mode: MirFuncImplMode::Normal,
            src_lineno_pseudo: 0,
        }
    }

    fn lifetimeable_output() -> MirType {
        let opaque = MirTypeRustOpaque {
            namespace: Namespace::default(),
            inner: MirRustOpaqueInner(MirLifetimeAwareType::new("crate::api::Handle".into())),
            codec: RustOpaqueCodecMode::Nom,
            dart_api_type: None,
            brief_name: false,
        };
        let raw = MirRustAutoOpaqueRaw {
            string: MirLifetimeAwareType::new("crate::api::Handle".into()),
            segments: vec![],
        };
        MirType::Delegate(MirTypeDelegate::Lifetimeable(MirTypeDelegateLifetimeable {
            api_type: MirTypeRustAutoOpaqueImplicit {
                ownership_mode: crate::codegen::ir::mir::func::OwnershipMode::Ref,
                inner: opaque.clone(),
                raw: raw.clone(),
                reason: None,
                ignore: false,
            },
            delegate: MirTypeDelegateRustAutoOpaqueExplicit { inner: opaque, raw },
        }))
    }

    /// Creates the exact lifetime-extension binding used by lockable decode paths.
    #[test]
    fn illegal_static_reference_uses_ouroboros_binding_for_requested_variable() {
        assert_eq!(
            generate_illegal_static_reference("api_handle"),
            "let api_handle_illegal_static_ref = unsafe {\n            flutter_rust_bridge::for_generated::ouroboros_change_lifetime(&api_handle)\n        };"
        );
    }

    /// Adds static references and argument substitution only for lifetime-bearing inputs.
    #[test]
    fn lifetime_decode_and_argument_paths_follow_needs_extend_lifetime() {
        let extended = func(true);
        assert!(generate_code_inner_decode(&extended, "decode();")
            .contains("api_handle_illegal_static_ref"));
        assert_eq!(
            generate_inner_func_arg("api_handle", &extended.inputs[0]),
            "api_handle_illegal_static_ref"
        );
        let ordinary = func(false);
        assert_eq!(
            generate_code_inner_decode(&ordinary, "decode();"),
            "decode();"
        );
        assert_eq!(
            generate_inner_func_arg("api_handle", &ordinary.inputs[0]),
            "api_handle"
        );
    }

    /// Wraps lifetimeable outputs with dependency guards and leaves ordinary outputs unchanged.
    #[test]
    fn lifetime_postprocess_emits_dependency_guards_only_for_lifetimeable_output() {
        let mut extended = func(true);
        extended.output.normal = lifetimeable_output();
        let output = generate_code_postprocess_inner_output(&extended);
        assert_eq!(output, "let output_ok = RustAutoOpaque::new(Lifetimeable::new(output_ok, vec![flutter_rust_bridge::for_generated::LifetimeableDependency::new_guard_lockable(\n                    Box::new(api_handle_guard.clone()),\n                    Box::new(api_handle.clone()),\n                )]));");
        assert_eq!(generate_code_postprocess_inner_output(&func(false)), "");
    }
}

fn is_interest_field(field: &MirFuncInput) -> bool {
    field.needs_extend_lifetime
}

fn get_variable_name(field: &MirFuncInput) -> String {
    field.inner.name.rust_style(true)
}

pub(crate) fn generate_inner_func_arg(raw: &str, field: &MirFuncInput) -> String {
    if is_interest_field(field) {
        format!("{raw}_illegal_static_ref")
    } else {
        raw.to_owned()
    }
}

pub(super) fn generate_code_postprocess_inner_output(func: &MirFunc) -> String {
    if !matches!(
        &func.output.normal,
        MirType::Delegate(MirTypeDelegate::Lifetimeable(_))
    ) {
        return "".to_owned();
    }

    let dependencies = (func.inputs.iter())
        .filter(|field| is_interest_field(field))
        .map(get_variable_name)
        .map(|field_name| {
            format!(
                "flutter_rust_bridge::for_generated::LifetimeableDependency::new_guard_lockable(
                    Box::new(api_{field_name}_guard.clone()),
                    Box::new(api_{field_name}.clone()),
                )"
            )
        })
        .join(", ");
    format!(
        "let output_ok = RustAutoOpaque::new(Lifetimeable::new(output_ok, vec![{dependencies}]));"
    )
}
