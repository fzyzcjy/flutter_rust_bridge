use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(frb))]
pub struct ModuleMeta {
    #[darling(default)]
    pub ignore: bool,
}

