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
