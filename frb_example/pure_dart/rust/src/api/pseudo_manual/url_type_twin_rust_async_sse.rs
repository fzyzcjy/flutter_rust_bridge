// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `url_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[derive(Debug, Clone)]
pub struct FeatureUrlTwinRustAsyncSse {
    pub one: url::Url,
}

#[derive(Debug, Clone)]
pub struct FeatureUriparseUriTwinRustAsyncSse {
    pub one: uriparse::URI<'static>,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_url_twin_rust_async_sse(url: url::Url) -> anyhow::Result<url::Url> {
    Ok(url)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_urls_twin_rust_async_sse(urls: Vec<url::Url>) -> anyhow::Result<Vec<url::Url>> {
    Ok(urls)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_nested_url_twin_rust_async_sse(
    url: FeatureUrlTwinRustAsyncSse,
) -> anyhow::Result<FeatureUrlTwinRustAsyncSse> {
    Ok(url)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_uriparse_uri_twin_rust_async_sse(
    uri: uriparse::URI<'static>,
) -> anyhow::Result<uriparse::URI<'static>> {
    Ok(uri)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_uriparse_uris_twin_rust_async_sse(
    uris: Vec<uriparse::URI<'static>>,
) -> anyhow::Result<Vec<uriparse::URI<'static>>> {
    Ok(uris)
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_nested_uriparse_uri_twin_rust_async_sse(
    uri: FeatureUriparseUriTwinRustAsyncSse,
) -> anyhow::Result<FeatureUriparseUriTwinRustAsyncSse> {
    Ok(uri)
}
