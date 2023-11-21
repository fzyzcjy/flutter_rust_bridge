use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};

mod c_binding;
mod dispatcher;
pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecMisc {
    pub(crate) c_binding: WireDartOutputCode,
    pub(crate) boilerplate: WireDartOutputCode,
    pub(crate) dispatcher_api_functions: Vec<WireDartOutputCode>,
    pub(crate) dispatcher_opaque_getters: Vec<WireDartOutputCode>,
    pub(crate) needs_freezed: bool,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
    c_file_content: &str,
) -> anyhow::Result<WireDartOutputSpecMisc> {
    Ok(WireDartOutputSpecMisc {
        c_binding: c_binding::generate(&context.config, c_file_content)?,
        boilerplate: generate_boilerplate(&context.config.dart_output_class_name_pack),
        dispatcher_api_functions: (context.ir_pack.funcs.iter())
            .map(|f| dispatcher::generate_dispatcher_api_function(f, context))
            .collect(),
        dispatcher_opaque_getters: (cache.distinct_types.iter())
            .filter_map(|ty| dispatcher::generate_dispatcher_opaque_getter(ty, context))
            .collect(),
        needs_freezed: compute_needs_freezed(cache, context.ir_pack),
    })
}

fn generate_boilerplate(
    dart_output_class_name_pack: &DartOutputClassNamePack,
) -> WireDartOutputCode {
    let DartOutputClassNamePack {
        entrypoint_class_name,
        api_class_name,
        api_impl_class_name,
        wire_class_name,
        ..
    } = &dart_output_class_name_pack;

    WireDartOutputCode {
        import: "import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';".into(),
        body_top: format!(
            r#"
            /// Main entrypoint of the Rust API
            class {entrypoint_class_name} extends BaseEntrypoint<{api_class_name}, {api_impl_class_name}, {wire_class_name}> {{
              @internal
              static final instance = {entrypoint_class_name}._();

              {entrypoint_class_name}._();

              /// Initialize flutter_rust_bridge
              static Future<void> init({{
                {api_class_name}? api,
                BaseHandler? handler,
              }}) async {{
                await instance.initImpl(api: api, handler: handler);
              }}
              
              /// Dispose flutter_rust_bridge
              ///
              /// The call to this function is optional, since flutter_rust_bridge (and everything else)
              /// is automatically disposed when the app stops.
              static void dispose() => instance.disposeImpl();

              @override
              ApiImplConstructor<{api_impl_class_name}, {wire_class_name}> get apiImplConstructor => {api_impl_class_name}.new;

              @override
              WireConstructor<{wire_class_name}> get wireConstructor => {wire_class_name}.new;

              @override
              String get defaultExternalLibraryStem => '{TODO}';
            }}
            "#
        ),
        ..Default::default()
    }
}

fn compute_needs_freezed(cache: &IrPackComputedCache, ir_pack: &IrPack) -> bool {
    cache
        .distinct_types
        .iter()
        .any(|ty| compute_needs_freezed_for_type(ty, ir_pack))
}

fn compute_needs_freezed_for_type(ty: &IrType, ir_pack: &IrPack) -> bool {
    match ty {
        EnumRef(_) => true,
        StructRef(st) => st.get(ir_pack).using_freezed(),
        _ => false,
    }
}
