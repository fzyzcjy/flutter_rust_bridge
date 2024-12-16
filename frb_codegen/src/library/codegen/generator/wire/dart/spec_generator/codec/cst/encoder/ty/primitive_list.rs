use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl<'a> WireDartCodecCstGeneratorEncoderTrait for PrimitiveListWireDartCodecCstGenerator<'a> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        // We do not care about codecov of unsupported things
        // frb-coverage:ignore-start
        if matches!(
            self.mir.primitive,
            MirTypePrimitive::Isize | MirTypePrimitive::Usize
        ) {
            return Acc::new_io_web(Some(
                "throw UnimplementedError('Not implemented in this codec');".to_owned(),
            ));
        }
        // frb-coverage:ignore-end

        Acc {
            // NOTE Dart code *only* allocates memory. It never *release* memory by itself.
            // Instead, Rust receives that pointer and now it is in control of Rust.
            // Therefore, *never* continue to use this pointer after you have passed the pointer
            // to Rust.
            // NOTE WARN: Never use the [calloc] provided by Dart FFI to allocate any memory.
            // Instead, ask Rust to allocate some memory and return raw pointers. Otherwise,
            // memory will be allocated in one dylib (e.g. libflutter.so), and then be released
            // by another dylib (e.g. my_rust_code.so), especially in Android platform. It can be
            // undefined behavior.
            io: Some(format!(
                "final ans = wire.cst_new_{}(raw.length);
                ans.ref.ptr.asTypedList(raw.length).setAll(0, {});
                return ans;",
                self.mir.safe_ident(),
                match self.mir.primitive {
                    MirTypePrimitive::I64 | MirTypePrimitive::U64 => "raw.inner",
                    _ => "raw",
                }
            )),
            web: Some(
                match self.mir.primitive {
                    MirTypePrimitive::I64 | MirTypePrimitive::U64 => "return raw.inner.jsify()!;",
                    _ => "return raw.jsify()!;",
                }
                .into(),
            ),
            ..Default::default()
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => {
                format!("ffi.Pointer<wire_cst_{}>", self.mir.safe_ident())
            }
            Target::Web => "JSAny".to_owned(),
        }
    }
}
