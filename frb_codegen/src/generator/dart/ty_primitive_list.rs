use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_dart_generator_struct;

type_dart_generator_struct!(TypePrimitiveListGenerator, IrTypePrimitiveList);

impl TypeDartGeneratorTrait for TypePrimitiveListGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
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
                "final ans = inner.new_{}_{}(raw.length);
                    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw{});
                    return ans;",
                self.ir.safe_ident(),
                self.context.config.block_index,
                match &self.ir.primitive {
                    IrTypePrimitive::I64 | IrTypePrimitive::U64 => ".inner",
                    _ => "",
                }
            )),
            wasm: Some("return raw;".into()),
            ..Default::default()
        }
    }

    fn wire2api_body(&self) -> String {
        match &self.ir.primitive {
            IrTypePrimitive::I64 => "return Int64List.from(raw);".into(),
            IrTypePrimitive::U64 => "return Uint64List.from(raw);".into(),
            _ => gen_wire2api_simple_type_cast(&self.ir.dart_api_type()),
        }
    }
}
