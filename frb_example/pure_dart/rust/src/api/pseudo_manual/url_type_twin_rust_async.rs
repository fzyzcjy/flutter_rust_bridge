// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `url_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[derive(Debug, Clone)]
pub struct FeatureUrlTwinRustAsync {
    pub one: url::Url,
}

#[derive(Debug, Clone)]
pub struct FeatureUriparseUriTwinRustAsync {
    pub one: uriparse::URI<'static>,
}

pub async fn handle_url_twin_rust_async(url: url::Url) -> anyhow::Result<url::Url> {
    Ok(url)
}

pub async fn handle_urls_twin_rust_async(urls: Vec<url::Url>) -> anyhow::Result<Vec<url::Url>> {
    Ok(urls)
}

pub async fn handle_nested_url_twin_rust_async(
    url: FeatureUrlTwinRustAsync,
) -> anyhow::Result<FeatureUrlTwinRustAsync> {
    Ok(url)
}

pub async fn handle_uriparse_uri_twin_rust_async(
    uri: uriparse::URI<'static>,
) -> anyhow::Result<uriparse::URI<'static>> {
    Ok(uri)
}

pub async fn handle_uriparse_uris_twin_rust_async(
    uris: Vec<uriparse::URI<'static>>,
) -> anyhow::Result<Vec<uriparse::URI<'static>>> {
    Ok(uris)
}

pub async fn handle_nested_uriparse_uri_twin_rust_async(
    uri: FeatureUriparseUriTwinRustAsync,
) -> anyhow::Result<FeatureUriparseUriTwinRustAsync> {
    Ok(uri)
}
