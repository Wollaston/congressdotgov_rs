use crate::{
    api::ApiError,
    client::Client,
    params::QueryParams,
    query::{self, Query},
};
use http::{Method, Request};
use serde::de::DeserializeOwned;
use std::{borrow::Cow, fmt::Display};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlBase {
    ApiV3,
}

impl UrlBase {
    pub fn endpoint_for<C>(&self, client: &C, endpoint: &str) -> Result<Url, ApiError<C::Error>>
    where
        C: Client,
    {
        match self {
            UrlBase::ApiV3 => client.rest_endpoint(endpoint),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Json,
    Xml,
}

impl Default for Format {
    fn default() -> Self {
        Self::Json
    }
}

impl From<Format> for Cow<'static, str> {
    fn from(format: Format) -> Self {
        format.into()
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Xml => write!(f, "xml"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    Asc,
    Desc,
}

impl Default for Sort {
    fn default() -> Self {
        Self::Asc
    }
}

impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Asc => write!(f, "asc"),
            Self::Desc => write!(f, "desc"),
        }
    }
}

pub trait Endpoint {
    fn method(&self) -> Method;

    fn endpoint(&self) -> Cow<'static, str>;

    fn url_base(&self) -> UrlBase {
        UrlBase::ApiV3
    }

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
