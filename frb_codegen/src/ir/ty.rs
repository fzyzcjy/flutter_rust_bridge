use crate::{ir::*, target::Target};
use enum_dispatch::enum_dispatch;
use IrType::*;

/// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(IrTypeTrait)]
#[derive(Debug, Clone, PartialEq)]
pub enum IrType {
    Primitive(IrTypePrimitive),
    Delegate(IrTypeDelegate),
    PrimitiveList(IrTypePrimitiveList),
    Optional(IrTypeOptional),
    GeneralList(IrTypeGeneralList),
    StructRef(IrTypeStructRef),
    Boxed(IrTypeBoxed),
    EnumRef(IrTypeEnumRef),
    SyncReturn(IrTypeSyncReturn),
    Opaque(IrTypeOpaque),
}

impl IrType {
    pub fn visit_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        if f(self) {
            return;
        }

        self.visit_children_types(f, ir_file);
    }

    #[inline]
    pub fn dart_required_modifier(&self) -> &'static str {
        match self {
            Optional(_) => "",
            _ => "required ",
        }
    }

    /// Additional indirection for types put behind a vector
    #[inline]
    pub fn rust_ptr_modifier(&self) -> &'static str {
        match self {
            Optional(_) | Delegate(IrTypeDelegate::String) => "*mut ",
            _ => "",
        }
    }

    #[inline]
    pub fn as_primitive(&self) -> Option<&IrTypePrimitive> {
        match self {
            Primitive(repr) | Delegate(IrTypeDelegate::PrimitiveEnum { repr, .. }) => Some(repr),
            _ => None,
        }
    }

    #[inline]
    pub fn is_primitive(&self) -> bool {
        self.as_primitive().is_some()
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, IrType::Delegate(IrTypeDelegate::Array(_),))
    }

    #[inline]
    pub fn is_struct(&self) -> bool {
        matches!(self, StructRef(_) | EnumRef(_))
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        matches!(self, Opaque(_))
    }

    // todo rework
    #[inline]
    pub fn contains_opaque(&self, ir_file: &IrFile) -> bool {
        #[derive(PartialEq, Debug, Clone, Copy)]
        enum Check {
            T,
            F,
            S,
        }

        let mut checked = vec![];
        fn temp(ty: &IrType, ir_file: &IrFile, ch: &mut Vec<(IrType, Check)>) -> Check {
            if let Some((_, res)) = ch.iter().find(|(t, _)| t == ty) {
                return *res;
            }
            ch.push((ty.clone(), Check::S));

            match ty {
                Optional(o) => temp(&o.inner, ir_file, ch),
                GeneralList(g) => temp(&g.inner, ir_file, ch),
                StructRef(s) => {
                    let check: Vec<Check> = s
                        .get(ir_file)
                        .fields
                        .iter()
                        .map(|f| temp(&f.ty, ir_file, ch))
                        .collect();
                    if check.iter().any(|c| *c == Check::T) {
                        return Check::T;
                    } else {
                        return Check::F;
                    }
                }
                Boxed(b) => temp(&b.inner, ir_file, ch),
                EnumRef(e) => {
                    let checks: Vec<Check> = e
                        .get(ir_file)
                        .variants()
                        .iter()
                        .map(|v| match &v.kind {
                            IrVariantKind::Value => Check::F,
                            IrVariantKind::Struct(s) => {
                                let check: Vec<Check> =
                                    s.fields.iter().map(|f| temp(&f.ty, ir_file, ch)).collect();
                                if check.iter().any(|c| *c == Check::T) {
                                    return Check::T;
                                } else {
                                    return Check::F;
                                }
                            }
                        })
                        .collect();
                    if checks.iter().any(|c| *c == Check::T) {
                        return Check::T;
                    } else {
                        return Check::F;
                    }
                }
                Opaque(_) => Check::T,
                Delegate(d) => match d {
                    IrTypeDelegate::Array(a) => match a {
                        IrTypeDelegateArray::GeneralArray { general, .. } => {
                            return temp(general, ir_file, ch)
                        }
                        IrTypeDelegateArray::PrimitiveArray { .. } => return Check::F,
                    },
                    _ => return Check::F,
                },
                _ => Check::F,
            }
        }

        let tt = temp(self, ir_file, &mut checked);
        println!("{} - {:?}", self.rust_api_type(), tt);
        match tt {
            Check::T => true,
            Check::F => false,
            Check::S => unreachable!(),
        }
    }

    /// In WASM, these types belong to the JS scope-local heap, **NOT** the Rust heap
    /// and therefore do not implement [Send].
    #[inline]
    pub fn is_js_value(&self) -> bool {
        match self {
            Self::GeneralList(_)
            | Self::StructRef(_)
            | Self::EnumRef(_)
            | Self::Delegate(IrTypeDelegate::PrimitiveEnum { .. }) => true,
            Self::Boxed(IrTypeBoxed { inner, .. }) => inner.is_js_value(),
            _ => false,
        }
    }
}

#[enum_dispatch]
pub trait IrTypeTrait {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile);

    fn safe_ident(&self) -> String;

    fn dart_api_type(&self) -> String;

    fn dart_wire_type(&self, target: Target) -> String;

    fn rust_api_type(&self) -> String;

    fn rust_wire_type(&self, target: Target) -> String;

    fn rust_wire_modifier(&self, target: Target) -> String {
        if self.rust_wire_is_pointer(target) {
            "*mut ".to_string()
        } else {
            "".to_string()
        }
    }

    fn rust_wire_is_pointer(&self, _target: Target) -> bool {
        false
    }

    fn dart_param_type(&self) -> &'static str {
        "dynamic"
    }
}

pub fn optional_boundary_index(types: &[&IrType]) -> Option<usize> {
    types
        .iter()
        .enumerate()
        .find(|ty| matches!(ty.1, Optional(_)))
        .and_then(|(idx, _)| {
            types[idx..]
                .iter()
                .all(|ty| matches!(ty, Optional(_)))
                .then_some(idx)
        })
}
