use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypeOpaqueGenerator, IrTypeOpaque);

impl TypeDartGeneratorTrait for TypeOpaqueGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(format!(
                "final ptr = inner.new_{0}();
                _api_fill_to_wire_{0}(raw, ptr);
                return ptr;",
                self.ir.safe_ident(),
            )),
            wasm: Some("return raw.tryShare();".to_owned()),
            ..Default::default()
        }
    }

    fn api_validate(&self) -> Option<String> {
        Some(
            "if (raw.isStale()) {
            throw StateError('Use after dispose.');
            }"
            .to_owned(),
        )
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some("wireObj.ptr = apiObj.tryShare();".into())
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return {}.fromRaw(raw[0], raw[1]);",
            self.ir.dart_api_type()
        )
    }

    fn structs(&self) -> String {
        format!(
            "@sealed class {0} extends FrbOpaque {{
                    static final dynamic _finalizer = FrbOpaque.createFinalizer(inner_platform.get_finalizer_opaque_{0}());
                    {0}.fromRaw(int ptr, int size) : super.unsafe(ptr) {{
                      FrbOpaque.attachFinalizer(_finalizer, ptr, this, size);
                    }}
                    
                    @override
                    void drop(ptr) {{
                      FrbOpaque.detachFinalizer(_finalizer, this);
                      inner_platform.inner.drop_opaque_{0}(ptr);
                    }}
                    
                    @override
                    share(ptr) => inner_platform.inner.share_opaque_{0}(ptr);
            }}",
            self.ir.dart_api_type()
        )
    }
}
