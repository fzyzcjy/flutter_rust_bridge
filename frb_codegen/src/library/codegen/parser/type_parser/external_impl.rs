use anyhow::Result;
use syn::{Type, visit_mut::VisitMut};

pub(crate) fn parse_type(ty: Type) -> Result<Type> {
    struct Visitor;
    impl VisitMut for Visitor {
        fn visit_expr_mut(&mut self, node: &mut Expr) {
            if let Expr::Lit(expr) = &node {
                if let Lit::Int(int) = &expr.lit {
                    if int.suffix() == "u256" {
                        let digits = int.base10_digits();
                        let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
                        *node = parse_quote!(bigint::u256!(#unsuffixed));
                        return;
                    }
                }
            }

            // Delegate to the default impl to visit nested expressions.
            visit_mut::visit_expr_mut(self, node);
        }
    }
    Visitor.visit_type_mut(ty);
    ty
}

pub(crate) fn parse_name(raw_name: &str) -> Result<String> {
    const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";
    Ok(if raw_name.starts_with(DUMMY_STRUCT_PREFIX) {
        String::from_utf8(hex::decode(&raw_name[DUMMY_STRUCT_PREFIX.len()..])?)?
    } else {
        raw_name.to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_external_impl_dummy_struct_name() {
        assert_eq!(parse_name("One<Two,Three>").unwrap(), "One<Two,Three>");
        assert_eq!(
            parse_name("__external_impl__4f6e65203c2054776f2c205468726565203e").unwrap(),
            "One < Two, Three >".to_owned(),
        );
    }
}
