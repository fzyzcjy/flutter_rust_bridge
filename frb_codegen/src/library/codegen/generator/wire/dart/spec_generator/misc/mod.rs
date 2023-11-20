use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};

pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecMisc {
    pub(crate) boilerplate: WireDartOutputCode,
    pub(crate) needs_freezed: bool,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecMisc {
    WireDartOutputSpecMisc {
        boilerplate: generate_boilerplate(&context.config.dart_entrypoint_class_name).into(),
        needs_freezed: compute_needs_freezed(cache, context.ir_pack),
    }
}

fn generate_boilerplate(entrypoint_class_name: &str) -> String {
    let dispatcher_name = format!("{}Dispatcher", entrypoint_class_name);

    format!(
        r#"
        /// Main entrypoint of the Rust API
        class {entrypoint_class_name} extends BaseEntrypoint<{dispatcher_name}> {{
          @internal
          static final instance = {entrypoint_class_name}._();

          {entrypoint_class_name}._();

          static Future<void> init({{
            {dispatcher_name}? dispatcher,
          }}) async {{
            await instance.initImpl(dispatcher: dispatcher ?? {dispatcher_name}());
          }}
        }}
        
        class {dispatcher_name} extends BaseDispatcher {{
          {dispatcher_name}({{super.handler}});

          // TODO
        }}
        "#
    )
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
