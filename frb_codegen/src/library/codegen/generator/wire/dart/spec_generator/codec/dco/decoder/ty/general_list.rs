use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for GeneralListWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        if let MirType::Delegate(MirTypeDelegate::Uuid) = &*self.mir.inner {
            return "const kUuidSizeInBytes = 16;
                final bytes = dco_decode_list_prim_u_8_strict(raw);
                return List.generate(
                  bytes.lengthInBytes ~/ kUuidSizeInBytes,
                  (i) => UuidValue.fromByteList(Uint8List.view(bytes.buffer, i * kUuidSizeInBytes, kUuidSizeInBytes)),
                  growable: false,
                );".to_owned();
        }

        format!(
            "return (raw as List<dynamic>).map(dco_decode_{}).toList();",
            self.mir.inner.safe_ident()
        )
    }
}
