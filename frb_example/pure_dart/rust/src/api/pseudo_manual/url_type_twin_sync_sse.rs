// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `url_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[derive(Debug, Clone)]
pub struct FeatureUrlTwinSyncSse {
    pub one: url::Url,
}

#[derive(Debug, Clone)]
pub struct FeatureUriparseUriTwinSyncSse {
    pub one: uriparse::URI<'static>,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_url_twin_sync_sse(url: url::Url) -> anyhow::Result<url::Url> {
    Ok(url)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_urls_twin_sync_sse(urls: Vec<url::Url>) -> anyhow::Result<Vec<url::Url>> {
    Ok(urls)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_nested_url_twin_sync_sse(
    url: FeatureUrlTwinSyncSse,
) -> anyhow::Result<FeatureUrlTwinSyncSse> {
    Ok(url)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_uriparse_uri_twin_sync_sse(
    uri: uriparse::URI<'static>,
) -> anyhow::Result<uriparse::URI<'static>> {
    Ok(uri)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_uriparse_uris_twin_sync_sse(
    uris: Vec<uriparse::URI<'static>>,
) -> anyhow::Result<Vec<uriparse::URI<'static>>> {
    Ok(uris)
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_nested_uriparse_uri_twin_sync_sse(
    uri: FeatureUriparseUriTwinSyncSse,
) -> anyhow::Result<FeatureUriparseUriTwinSyncSse> {
    Ok(uri)
}
