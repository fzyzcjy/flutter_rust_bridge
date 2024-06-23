use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::Namespace;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for DelegateWireRustGenerator<'a> {
    fn generate_imports(&self) -> Option<Vec<Namespace>> {
        if let MirTypeDelegate::CustomSerDes(mir) = &self.mir {
            Some(
                [&mir.info.rust2dart, &mir.info.dart2rust]
                    .into_iter()
                    .map(|x| x.rust_function.namespace.clone())
                    .collect_vec(),
            )
        } else {
            None
        }
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn wrapper_struct_name(&self) -> Option<String> {
        // frb-coverage:ignore-end
        if let MirTypeDelegate::PrimitiveEnum(enu) = &self.mir {
            WireRustGenerator::new(enu.mir.clone(), self.context).wrapper_struct_name()
        } else {
            None
        }
    }

    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        match &self.mir {
            MirTypeDelegate::ProxyEnum(mir) => Some(mir.get_delegate().rust_api_type()),
            MirTypeDelegate::DynTrait(mir) => Some(mir.get_delegate().rust_api_type()),
            MirTypeDelegate::Lifetimeable(mir) => Some(mir.delegate.inner.rust_api_type()),
            _ => None,
        }
    }
}
