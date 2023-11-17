pub(super) fn get_into_into_dart(name: &str, wrapper_name: Option<&str>) -> String {
    match wrapper_name {
        None => {
            // case for types without mirror_ wrapper
            format!(
                "impl rust2dart::IntoIntoDart<{name}> for {name} {{
                fn into_into_dart(self) -> Self {{
                    self
                }}
            }}"
            )
        }
        Some(wrapper_name) => {
            // case for type with mirror_ wrapper
            format!(
                "impl rust2dart::IntoIntoDart<{wrapper_name}> for {name} {{
                fn into_into_dart(self) -> {wrapper_name} {{
                    {wrapper_name}(self)
                }}
            }}"
            )
        }
    }
}
