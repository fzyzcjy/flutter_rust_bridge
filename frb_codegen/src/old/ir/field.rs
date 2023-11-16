use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use syn::LitStr;

use crate::{ir::*, parser::IrDefaultValue, Opts};

impl IrField {
    fn default_value_to_dart_style(lit: &LitStr) -> String {
        let value = lit.value();
        let mut split = value.split('.');
        let enum_name = split.next().unwrap();

        let variant_name = split.next().unwrap().to_string();
        let variant_name =
            crate::utils::misc::make_string_keyword_safe(variant_name.to_case(Case::Camel));

        format!("{enum_name}.{variant_name}")
    }
}
