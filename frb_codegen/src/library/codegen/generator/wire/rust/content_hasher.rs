use crate::codegen::generator::acc::Acc;
use sha1::Digest;

pub(crate) const CONTENT_HASH_PLACEHOLDER: &str = "__PLACEHOLDER_FRB_CONTENT_HASH__";

pub(crate) fn text_inject_content_hash(
    text: &Acc<Option<String>>,
    content_hash: i32,
) -> Acc<Option<String>> {
    text.clone()
        .map(|x, _| x.map(|x| x.replace(CONTENT_HASH_PLACEHOLDER, &content_hash.to_string())))
}

pub(crate) fn compute_content_hash(text: &Acc<Option<String>>) -> i32 {
    let interest_text = text.common.as_deref().unwrap_or("");
    let digest = sha1::Sha1::digest(interest_text.as_bytes());
    i32::from_le_bytes(digest[..4].try_into().unwrap())
}
