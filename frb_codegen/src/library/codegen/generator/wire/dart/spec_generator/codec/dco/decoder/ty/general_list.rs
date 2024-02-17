use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for GeneralListWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        if let IrType::Delegate(IrTypeDelegate::Uuid) = &*self.ir.inner {
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
            self.ir.inner.safe_ident()
        )
    }
}
