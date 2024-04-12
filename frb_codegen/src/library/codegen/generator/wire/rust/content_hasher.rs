use crate::codegen::generator::acc::Acc;
use itertools::Itertools;

pub(crate) const CONTENT_HASH_PLACEHOLDER: &str = "__PLACEHOLDER_FRB_CONTENT_HASH__";

pub(crate) fn text_inject_content_hash(
    text: &Acc<Option<String>>,
    content_hash: i32,
) -> Acc<Option<String>> {
    text.map(|x, _| x.map(|x| x.replace(CONTENT_HASH_PLACEHOLDER, content_hash)))
}

pub(crate) fn compute_content_hash(text: &Acc<Option<String>>) -> i32 {
    let joined_text = (text.clone().into_vec().into_iter())
        .filter_map(|x| x)
        .join("\n");
    let digest = sha1::Sha1::digest(joined_text.as_bytes());
    i32::from_le_bytes(digest[..4].try_into().unwrap())
}
