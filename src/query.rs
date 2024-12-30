use crate::{api::error::ApiError, cdg::CdgError, client::Client};
use http::Uri;
use url::Url;

/// A helper function for parsing a URL to an HTTP URI.
pub fn url_to_http_uri(url: Url) -> Result<Uri, CdgError> {
    Ok(url.as_str().parse::<Uri>()?)
}

/// A trait which represents a query which may be made to a congress.gov client.
pub trait Query<T, C>
where
    C: Client,
{
    /// Perform the query against the client.
    async fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;
}
