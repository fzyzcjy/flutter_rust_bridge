use crate::generator::dart::ty::TypeDartGeneratorTrait;
use crate::ir::*;

#[derive(Debug, Clone)]
pub struct TypeGeneralListGenerator(pub IrTypeGeneralList);

impl TypeDartGeneratorTrait for TypeGeneralListGenerator {
    fn api2wire_body(&self) -> String {
        // NOTE the memory strategy is same as PrimitiveList, see comments there.
        format!(
            "final ans = inner.new_{}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;",
            self.0.safe_ident(),
            self.0.inner.safe_ident()
        )
    }

    fn wire2api_body(&self) -> String {
        format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            self.0.inner.safe_ident()
        )
    }
}
