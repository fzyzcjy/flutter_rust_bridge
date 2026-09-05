use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::comments::generate_codec_comments;

pub(crate) fn generate_impl_into_dart(name: &str, body: &str) -> String {
    let codec_comments = generate_codec_comments(CodecMode::Dco);
    format!(
        "{codec_comments}
            impl flutter_rust_bridge::IntoDart for {name} {{
                fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {{
                    {body}
                }}
            }}
            impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for {name} {{}}
"
    )
}

pub(crate) fn generate_impl_into_into_dart(name: &str, wrapper_name: &Option<String>) -> String {
    let body = if wrapper_name.is_some() {
        "self.into()".into()
    } else {
        "self".to_owned()
    };

    let wrapper_name = wrapper_name.clone().unwrap_or(name.to_owned());

    format!(
        "impl flutter_rust_bridge::IntoIntoDart<{wrapper_name}> for {name} {{
            fn into_into_dart(self) -> {wrapper_name} {{
                {body}
            }}
        }}
"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Emits the DCO conversion traits with the supplied conversion body.
    #[test]
    fn generate_impl_into_dart_preserves_name_body_and_codec_marker() {
        let output = generate_impl_into_dart("ApiType", "convert(self)");

        assert!(output.contains("// Codec=Dco (DartCObject based)"));
        assert!(output.contains("impl flutter_rust_bridge::IntoDart for ApiType"));
        assert!(output.contains("convert(self)"));
        assert!(output.contains("IntoDartExceptPrimitive for ApiType"));
    }

    /// Selects identity conversion unless a wrapper type is requested.
    #[test]
    fn generate_impl_into_into_dart_selects_identity_or_wrapper_conversion() {
        let identity = generate_impl_into_into_dart("ApiType", &None);
        let wrapped = generate_impl_into_into_dart("ApiType", &Some("WireType".into()));

        assert!(identity.contains("IntoIntoDart<ApiType> for ApiType"));
        assert!(identity.contains("fn into_into_dart(self) -> ApiType {\n                self"));
        assert!(wrapped.contains("IntoIntoDart<WireType> for ApiType"));
        assert!(
            wrapped.contains("fn into_into_dart(self) -> WireType {\n                self.into()")
        );
    }
}
