use crate::{api::error::ApiError, cdg::CdgError, client::Client};
use http::Uri;
use url::Url;

pub fn url_to_http_uri(url: Url) -> Result<Uri, CdgError> {
    Ok(url.as_str().parse::<Uri>()?)
}

pub trait Query<T, C>
where
    C: Client,
{
    async fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
