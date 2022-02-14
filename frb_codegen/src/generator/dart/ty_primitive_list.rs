use crate::generator::dart::gen_wire2api_simple_type_cast;
use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypePrimitiveListGenerator(pub IrTypePrimitiveList);

impl TypeDartGeneratorTrait for TypePrimitiveListGenerator {
    fn api2wire_body(&self) -> String {
        // NOTE Dart code *only* allocates memory. It never *release* memory by itself.
        // Instead, Rust receives that pointer and now it is in control of Rust.
        // Therefore, *never* continue to use this pointer after you have passed the pointer
        // to Rust.
        // NOTE WARN: Never use the [calloc] provided by Dart FFI to allocate any memory.
        // Instead, ask Rust to allocate some memory and return raw pointers. Otherwise,
        // memory will be allocated in one dylib (e.g. libflutter.so), and then be released
        // by another dylib (e.g. my_rust_code.so), especially in Android platform. It can be
        // undefined behavior.
        format!(
            "final ans = inner.new_{}(raw.length);
                ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
                return ans;",
            self.0.safe_ident(),
        )
    }

    fn wire2api_body(&self) -> String {
        gen_wire2api_simple_type_cast(&self.0.dart_api_type())
    }
}
