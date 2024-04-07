pub(crate) fn parse_external_impl_dummy_struct_name(raw_name: &str) -> Option<String> {
    const DUMMY_STRUCT_PREFIX: &str = "__external_impl__";
    (raw_name.starts_with(DUMMY_STRUCT_PREFIX))
        .then(|| hex::decode(&raw_name[DUMMY_STRUCT_PREFIX.len()..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_external_impl_dummy_struct_name() {
        assert_eq!(
            parse_external_impl_dummy_struct_name("One<Two,Three>"),
            None,
        );
        assert_eq!(
            parse_external_impl_dummy_struct_name(
                "__external_impl__4f6e65203c2054776f2c205468726565203e"
            ),
            Some("One<Two,Three>".to_owned()),
        );
    }
}
