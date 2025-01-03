use http::{Method, Request};
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use url::Url;

use crate::{api::client::Client, api::error::ApiError, api::params::QueryParams};

use super::{query, Query};

/// The URL base for the congress.gov REST API.
/// Currently, there is only one variant for the current
/// V3 API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlBase {
    ApiV3,
}

impl UrlBase {
    /// Get the endpoint for a given URL base.
    pub fn endpoint_for<C>(&self, client: &C, endpoint: &str) -> Result<Url, ApiError<C::Error>>
    where
        C: Client,
    {
        match self {
            UrlBase::ApiV3 => client.rest_endpoint(endpoint),
        }
    }
}

/// A trait for providing the necessary information for a single REST API endpoint.
pub trait Endpoint {
    /// The HTTP method for the endpoint, e.g. GET.
    fn method(&self) -> Method;

    /// The path for the endpoint.
    fn endpoint(&self) -> Cow<'static, str>;

    /// The URL base for the endpoint, defaulting to the
    /// current V3 URL base.
    fn url_base(&self) -> UrlBase {
        UrlBase::ApiV3
    }

    /// Query parameters for the endpoint.
    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    async fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut url = self.url_base().endpoint_for(client, &self.endpoint())?;
        self.parameters().add_to_url(&mut url);
        client.set_auth(&mut url);

        let req = Request::builder()
            .method(self.method())
            .uri(query::url_to_http_uri(url).map_err(|e| ApiError::Cdg { source: e })?);

        let rsp = client.rest(req, Vec::new()).await?;

        let status = rsp.status();

        let val = if let Ok(val) = serde_json::from_slice(rsp.body()) {
            val
        } else {
            return Err(ApiError::Http { status });
        };

        if !status.is_success() {
            return Err(ApiError::Http { status });
        }

        serde_json::from_value::<T>(val).map_err(|e| ApiError::DataType { source: e })
    }
}
