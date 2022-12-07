use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrTypeImplTrait {
    pub trait_bounds: Vec<String>,
}

impl IrTypeImplTrait {
    // use _ just to connect bound, You can think of it as non-lowercase snake
    pub fn to_enum(&self) -> String {
        format!("{}Enum", self.trait_bounds.join("_")).to_case(Case::Pascal)
    }

    pub fn to_enum_ir_type(&self) -> IrType {
        IrType::EnumRef(IrTypeEnumRef {
            name: self.to_enum(),
        })
    }
}

impl std::fmt::Display for IrTypeImplTrait {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fmt.write_str(&self.to_enum())
    }
}

impl IrTypeImplTrait {
    pub fn new_raw(raw: Vec<String>) -> IrTypeImplTrait {
        IrTypeImplTrait { trait_bounds: raw }
    }
}

impl IrTypeTrait for IrTypeImplTrait {
    fn visit_children_types<F: FnMut(&super::IrType) -> bool>(
        &self,
        f: &mut F,
        ir_file: &super::IrFile,
    ) {
        // When TraitBoundEnum automatic generated,
        // There is no need to rely on each trait in TraitBound, or anything.
        // There is also no children.
        // You can think of it(TypeImplTrait) as just an annotation.
        //
        // But When generated code about func dependencies(generate function arguments).
        // We need generate code that is the same as Enum.
        if ir_file.enum_pool.contains_key(&self.to_enum()) {
            IrTypeEnumRef {
                name: self.to_enum(),
            }
            .visit_children_types(f, ir_file)
        }
    }

    fn safe_ident(&self) -> String {
        self.to_enum().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        self.rust_api_type()
    }

    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.to_enum()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.to_enum())
        }
    }
}
