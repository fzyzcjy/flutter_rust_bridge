use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;

impl<'a> WireRustGeneratorApi2wireTrait for OptionalWireRustGenerator<'a> {
    fn generate_convert_to_dart(&self, obj: String) -> String {
        let inner = WireRustGenerator::new(self.ir.inner.clone(), self.context);
        let obj = match inner.wrapper_struct_name() {
            // An architecture has been created so that the inner type of optional field is always
            // IrTypeBoxed. Here, too, if we use inner.self_access("v".to_owned()), since it will go to
            // self_access in TypeBoxedGenerator, the dereferenced value *v was returned from there, which
            // gave the error that the external struct could not be dereferenced.
            //
            // For now, removed the parameter inner.self_access("v".to_owned()) and then added a bit of a
            // hacky method here (not used self_access function), as it only covers mirrored optional fields.
            Some(wrapper) => format!("{}.map(|v| {}(v))", obj, wrapper),
            None => obj,
        };
        format!("{obj}.into_dart()")
    }
}
