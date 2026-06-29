use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::{MirType, MirTypeTrait};

impl WireDartCodecCstGeneratorEncoderTrait for GeneralListWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        let ident = self.mir.safe_ident();
        let inner = self.mir.inner.safe_ident();

        Acc {
            io: Some(format!(
                "final ans = wire.cst_new_{ident}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    {}
                }}
                return ans;
                ",
                if general_list_inner_uses_direct_assignment(&self.mir.inner) {
                    format!("ans.ref.ptr[i] = cst_encode_{inner}(raw[i]);")
                } else {
                    format!("cst_api_fill_to_wire_{inner}(raw[i], ans.ref.ptr[i]);")
                }
            )),
            web: self.context.config.web_enabled.then(|| {
                format!(
                    "return raw.map(cst_encode_{}).toList().jsify()!;",
                    self.mir.inner.safe_ident()
                )
            }),
            ..Default::default()
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => format!("ffi.Pointer<wire_cst_{}>", self.mir.safe_ident()),
            Target::Web => "JSAny".into(),
        }
    }
}

fn general_list_inner_uses_direct_assignment(inner: &MirType) -> bool {
    inner.is_primitive()
        || matches!(
            inner,
            MirType::Optional(_)
                | MirType::RustAutoOpaqueImplicit(_)
                | MirType::RustOpaque(_)
                | MirType::Delegate(
                    MirTypeDelegate::CastedPrimitive(_)
                        | MirTypeDelegate::RustAutoOpaqueExplicit(_)
                        | MirTypeDelegate::String
                        | MirTypeDelegate::StreamSink(_)
                        | MirTypeDelegate::Time(_)
                        | MirTypeDelegate::Uuid
                        | MirTypeDelegate::SerdeJsonValue
                )
                | MirType::DartOpaque(_)
                | MirType::PrimitiveList(_)
        )
}
