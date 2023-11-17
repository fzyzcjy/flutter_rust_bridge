pub(super) fn generate_impl_into_into_dart(name: &str, wrapper_name: Option<&str>) -> String {
    let wrapper_name = wrapper_name.unwrap_or(name);
    format!(
        "impl rust2dart::IntoIntoDart<{wrapper_name}> for {name} {{
            fn into_into_dart(self) -> {wrapper_name} {{
                {wrapper_name}(self)
            }}
        }}"
    )
}
