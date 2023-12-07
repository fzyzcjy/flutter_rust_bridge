use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

impl<'a> WireDartTransferCstGeneratorEncoderTrait for GeneralListWireDartTransferCstGenerator<'a> {
    fn encode_func_body(&self) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        let ident = self.ir.safe_ident();
        let inner = self.ir.inner.safe_ident();

        Acc {
            io: Some(format!(
                "final ans = wire.cst_new_{ident}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    {}
                }}
                return ans;
                ",
                if self.ir.inner.is_primitive()
                    || matches!(
                        *self.ir.inner,
                        IrType::RustOpaque(_) | IrType::DartOpaque(_)
                    )
                {
                    // Handle primitive enums list.
                    // This is similar to `StringList` in
                    // `frb_codegen/src/generator/dart/ty_delegate.rs`
                    format!("ans.ref.ptr[i] = cst_encode_{inner}(raw[i]);")
                } else {
                    format!("_cst_api_fill_to_wire_{inner}(raw[i], ans.ref.ptr[i]);")
                }
            )),
            wasm: self.context.config.wasm_enabled.then(|| {
                format!(
                    "return raw.map(cst_encode_{}).toList();",
                    self.ir.inner.safe_ident()
                )
            }),
            ..Default::default()
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        general_or_optional_list_dart_wire_type(target, &self.ir.clone().into())
    }
}

pub(super) fn general_or_optional_list_dart_wire_type(target: Target, ir: &IrType) -> String {
    match target {
        Target::Io => format!("ffi.Pointer<wire_cst_{}>", ir.safe_ident()),
        Target::Wasm => "List<dynamic>".into(),
    }
}
