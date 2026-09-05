use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::MirTypeTrait;

impl WireDartCodecCstGeneratorEncoderTrait for PrimitiveListWireDartCodecCstGenerator<'_> {
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
                    MirTypePrimitive::I64 => "return cstEncodeInt64List(raw.inner);",
                    MirTypePrimitive::U64 => "return cstEncodeUint64List(raw.inner);",
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

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use super::*;
    use crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;

    /// Covers typed-list allocation, 64-bit inner access, and unsupported pointer-sized values.
    #[test]
    fn primitive_list_encoder_covers_supported_and_unsupported_primitives() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(true);
        let wire_rust_config = test_utils::wire_rust_config(true);
        let context = test_utils::context(
            &pack,
            &wire_dart_config,
            &wire_rust_config,
            &api_dart_config,
        );
        let i64 = PrimitiveListWireDartCodecCstGenerator::new(
            MirTypePrimitiveList {
                primitive: MirTypePrimitive::I64,
                strict_dart_type: true,
            },
            context,
        )
        .generate_encode_func_body();
        let ordinary = PrimitiveListWireDartCodecCstGenerator::new(
            MirTypePrimitiveList {
                primitive: MirTypePrimitive::I32,
                strict_dart_type: true,
            },
            context,
        )
        .generate_encode_func_body();
        let unsupported = PrimitiveListWireDartCodecCstGenerator::new(
            MirTypePrimitiveList {
                primitive: MirTypePrimitive::Usize,
                strict_dart_type: true,
            },
            context,
        )
        .generate_encode_func_body();

        assert_eq!(
            i64.io.as_deref(),
            Some(
                "final ans = wire.cst_new_list_prim_i_64_strict(raw.length);\n                ans.ref.ptr.asTypedList(raw.length).setAll(0, raw.inner);\n                return ans;"
            )
        );
        assert_eq!(
            i64.web.as_deref(),
            Some("return cstEncodeInt64List(raw.inner);")
        );
        assert_eq!(
            ordinary.io.as_deref(),
            Some(
                "final ans = wire.cst_new_list_prim_i_32_strict(raw.length);\n                ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);\n                return ans;"
            )
        );
        assert_eq!(
            unsupported.io.as_deref(),
            Some("throw UnimplementedError('Not implemented in this codec');")
        );
    }
}
