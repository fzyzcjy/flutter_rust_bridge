use crate::codegen::generator::codec::structs::CodecMode;

pub(crate) fn generate_codec_comments(codec: CodecMode) -> String {
    let brief_explanation = match codec {
        CodecMode::Cst => "C-struct based",
        CodecMode::Dco => "DartCObject based",
        CodecMode::Sse => "Serialization based",
        CodecMode::Pde => "Serialization + dispatch",
    };
    format!("// Codec={codec} ({brief_explanation}), see doc to use other codecs")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_codec_comments() {
        assert_eq!(
            &generate_codec_comments(CodecMode::Pde),
            "// Codec=Pde (Serialization + dispatch), see doc to use other codecs",
        );
    }
}
