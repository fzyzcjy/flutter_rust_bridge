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

    /// Describes every supported codec with its stable generated-code comment.
    #[test]
    fn generates_comments_for_every_codec() {
        assert_eq!(
            generate_codec_comments(CodecMode::Cst),
            "// Codec=Cst (C-struct based), see doc to use other codecs",
        );
        assert_eq!(
            generate_codec_comments(CodecMode::Dco),
            "// Codec=Dco (DartCObject based), see doc to use other codecs",
        );
        assert_eq!(
            generate_codec_comments(CodecMode::Sse),
            "// Codec=Sse (Serialization based), see doc to use other codecs",
        );
        assert_eq!(
            generate_codec_comments(CodecMode::Pde),
            "// Codec=Pde (Serialization + dispatch), see doc to use other codecs",
        );
    }
}
