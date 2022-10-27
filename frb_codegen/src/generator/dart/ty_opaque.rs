use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeOpaqueGenerator, IrTypeOpaque);

impl TypeDartGeneratorTrait for TypeOpaqueGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "if (raw.isStale()) {{
                    throw 'Use after dispose.';
                  }}
                final ptr = inner.new_{0}();
                _api_fill_to_wire_{0}(raw, ptr);
                return ptr;",
                self.ir.safe_ident(),
            )),
            wasm: Some("return FrbOpaque.share(raw);".to_string()),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some("wireObj.ref.ptr = FrbOpaque.share(apiObj).cast();".into())
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return {}.fromRaw(raw[0], raw[1], raw[2], raw[3]);",
            self.ir.dart_api_type()
        )
    }

    fn structs(&self) -> String {
        format!(
            "@sealed class {0} extends FrbOpaque {{
                {0}.fromRaw(int ptr, int drop, int share, int size) : super.unsafe(ptr, drop, share, size);
            }}",
            self.ir.dart_api_type()
        )
    }
}
