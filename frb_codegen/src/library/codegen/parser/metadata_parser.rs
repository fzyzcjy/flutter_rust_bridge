fn parse_metadata(attrs: &[Attribute]) -> Vec<IrDartAnnotation> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("frb"))
        .map(|attr| attr.parse_args::<FrbOption>())
        .flat_map(|frb_option| match frb_option {
            Ok(FrbOption::Metadata(NamedOption {
                name: _,
                value: MetadataAnnotations(annotations),
            })) => annotations,
            _ => vec![],
        })
        .collect()
}
