use bytes::Bytes;
use http::{header::InvalidHeaderValue, Response};
use reqwest::{Client, Request};
use thiserror::Error;
use url::Url;

use crate::{
    api::{common::Format, ApiError},
    auth::Auth,
};

/// Various error types that can occur when using the Cdg struct
/// and its values.
#[derive(Debug, Error)]
pub enum CdgError {
    #[error("failed to parse url: {}", source)]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
    #[error("failed to parse uri: {}", source)]
    UriParse {
        #[from]
        source: http::uri::InvalidUri,
    },
    #[error("communication with gitlab: {}", source)]
    Communication {
        #[from]
        source: reqwest::Error,
    },
    #[error("HTTP Status Error: {}", status)]
    Status { status: http::StatusCode },
    #[error("HTTP Error: {}", source)]
    Http {
        #[from]
        source: http::Error,
    },
    #[error("could not parse data from JSON: {}", source)]
    DataType {
        #[from]
        source: serde_json::Error,
    },
    #[error("could not parse header value: {}", source)]
    Header {
        #[from]
        source: InvalidHeaderValue,
    },
}

/// The primary struct used when consuming Endpoints. Holds
/// a reusable reqwest::Client, base url, and Auth.
#[derive(Debug, Clone)]
pub struct Cdg {
    /// The HTTP client to use for API calls.
    pub client: Client,
    /// The base URL for API calls.
    pub base_url: Url,
    /// The authentication information to use when making API calls.
    pub auth: Auth,
    /// The desired response format. Can be XML or JSON.
    pub format: Format,
}

impl Cdg {
    /// Creates a new Cdg struct with the provided Auth.
    pub fn new(auth: Auth, client: reqwest::Client, format: Format) -> Result<Cdg, CdgError> {
        Ok(Cdg {
            client,
            base_url: Url::parse("https://api.congress.gov/v3/")?,
            auth,
            format,
        })
    }
}

impl crate::api::Client for Cdg {
    type Error = CdgError;

    /// Constructs an endpoint URL by combining the base_url
    /// with the provided endpoint.
    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        Ok(self.base_url.join(endpoint)?)
    }

    /// Appends the auth token to the URL for the request. The congress.gov
    /// API expects requests of the form https://api.congress.gov/v3/resource?api_key=[INSERT_KEY].
    fn set_auth(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        match &self.auth {
            Auth::Token(token) => pairs.extend_pairs([("api_key", token)]),
        };
    }

    /// Performs a REST API call for the given request and returns the response as Bytes.
    async fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>> {
        let call = || async {
            let http_request = request.body(body)?;
            let request: Request = http_request.try_into()?;
            let rsp = self.client.execute(request).await?;

            let mut http_rsp = Response::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            headers.insert(http::header::ACCEPT, self.format.as_header()?);
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }

            Ok(http_rsp.body(rsp.bytes().await?)?)
        };
        call().await.map_err(|e| ApiError::Client { source: e })
    }
}
