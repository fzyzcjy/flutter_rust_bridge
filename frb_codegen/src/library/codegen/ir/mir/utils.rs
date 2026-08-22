/// Adds some common derives for IR types.
///
/// Valid forms:
/// - `ir! { pub struct Foo { .. } .. }`
/// - `ir! { #[no_serde] pub struct Bar { .. } .. }`
#[doc(hidden)] // only to be used within crate, not for end users
#[macro_export]
macro_rules! mir {
    () => {};
    (#[no_serde] $decl:item $($rest:tt)*) => {
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        $decl

        $crate::mir!($($rest)*);
    };
    ($decl:item $($rest:tt)*) => {
        #[derive(Debug, Clone, Hash, Eq, PartialEq)]
        #[derive(::serde::Serialize)]
        $decl

        $crate::mir!($($rest)*);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    crate::mir! {
        struct SerializableMir {
            value: u8,
        }

        #[no_serde]
        struct NonSerializableMir {
            value: u8,
        }
    }

    /// The default macro branch adds serde and common value derives.
    #[test]
    fn mir_adds_serde_and_common_derives() {
        let value = SerializableMir { value: 7 };
        let mut values = HashSet::new();

        values.insert(value.clone());

        assert!(values.contains(&value));
        assert_eq!(serde_json::to_string(&value).unwrap(), r#"{"value":7}"#);
    }

    /// The no-serde macro branch still adds the common value derives.
    #[test]
    fn mir_no_serde_branch_adds_common_derives() {
        let value = NonSerializableMir { value: 9 };
        let mut values = HashSet::new();

        values.insert(value.clone());

        assert!(values.contains(&value));
    }
}
