#[derive(Debug, Clone)]
pub struct FeatureUrlTwinNormal {
    pub one: url::Url,
}

#[derive(Debug, Clone)]
pub struct FeatureUriparseUriTwinNormal {
    pub one: uriparse::URI<'static>,
}

pub fn handle_url_twin_normal(url: url::Url) -> anyhow::Result<url::Url> {
    Ok(url)
}

pub fn handle_urls_twin_normal(urls: Vec<url::Url>) -> anyhow::Result<Vec<url::Url>> {
    Ok(urls)
}

pub fn handle_nested_url_twin_normal(
    url: FeatureUrlTwinNormal,
) -> anyhow::Result<FeatureUrlTwinNormal> {
    Ok(url)
}

pub fn handle_uriparse_uri_twin_normal(
    uri: uriparse::URI<'static>,
) -> anyhow::Result<uriparse::URI<'static>> {
    Ok(uri)
}

pub fn handle_uriparse_uris_twin_normal(
    uris: Vec<uriparse::URI<'static>>,
) -> anyhow::Result<Vec<uriparse::URI<'static>>> {
    Ok(uris)
}

pub fn handle_nested_uriparse_uri_twin_normal(
    uri: FeatureUriparseUriTwinNormal,
) -> anyhow::Result<FeatureUriparseUriTwinNormal> {
    Ok(uri)
}
