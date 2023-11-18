pub(super) fn gen_wire2api_simple_type_cast(cast_type: &str) -> String {
    format!("return raw as {cast_type};")
}
