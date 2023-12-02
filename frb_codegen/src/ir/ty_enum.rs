use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

crate::ir! {
pub struct IrTypeEnumRef {
    pub name: String,
    pub is_exception: bool,
}
}

impl IrTypeEnumRef {
    #[inline]
    pub fn get<'a>(&self, ir_file: &'a IrFile) -> &'a IrEnum {
        &ir_file.enum_pool[&self.name]
    }
}

impl IrTypeTrait for IrTypeEnumRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        let enu = self.get(ir_file);
        for variant in enu.variants() {
            if let IrVariantKind::Struct(st) = &variant.kind {
                st.fields
                    .iter()
                    .for_each(|field| field.ty.visit_self_types_recursively(f, ir_file));
            }
        }
    }

    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        self.name.to_string()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.name.to_string()
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.name)
        }
    }

    fn intodart_type(&self, ir_file: &IrFile) -> String {
        match &self.get(ir_file).wrapper_name {
            Some(wrapper) => wrapper.clone(),
            None => self.dart_api_type(),
        }
    }
}

crate::ir! {
pub struct IrEnum {
    pub name: String,
    pub wrapper_name: Option<String>,
    pub path: Vec<String>,
    pub comments: Vec<IrComment>,
    pub is_exception: bool,
    variants: Vec<IrVariant>,
    is_struct: bool,
}
}

impl IrEnum {
    pub fn new(
        name: String,
        wrapper_name: Option<String>,
        path: Vec<String>,
        comments: Vec<IrComment>,
        mut variants: Vec<IrVariant>,
        is_exception: bool,
    ) -> Self {
        fn wrap_box(ty: &mut IrType) {
            if ty.is_struct_ref_or_enum_ref_or_record() {
                *ty = IrType::Boxed(IrTypeBoxed {
                    exist_in_real_api: false,
                    inner: Box::new(ty.clone()),
                });
            }
        }

        let is_struct = variants
            .iter()
            .any(|variant| !matches!(variant.kind, IrVariantKind::Value));
        if is_struct {
            for variant in &mut variants {
                if let IrVariantKind::Struct(st) = &mut variant.kind {
                    for field in &mut st.fields {
                        wrap_box(&mut field.ty);
                    }
                }
            }
        }
        Self {
            name,
            wrapper_name,
            path,
            comments,
            variants,
            is_struct,
            is_exception,
        }
    }

    pub fn variants(&self) -> &[IrVariant] {
        &self.variants
    }

    pub fn is_struct(&self) -> bool {
        self.is_struct
    }
}
crate::ir! {
pub struct IrVariant {
    pub name: IrIdent,
    pub wrapper_name: IrIdent,
    pub comments: Vec<IrComment>,
    pub kind: IrVariantKind,
}

pub enum IrVariantKind {
    Value,
    Struct(IrStruct),
}
}
