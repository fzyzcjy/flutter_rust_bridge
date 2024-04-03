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
