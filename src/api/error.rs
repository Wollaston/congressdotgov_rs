//! API Errors.

use std::error::Error;
use thiserror::Error;

use crate::cdg::CdgError;

/// Error types that can be returned or occur when communicating with the
/// congress.gov api.
#[derive(Debug, Error)]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    #[error("Client Error: {}", source)]
    Client { source: E },
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
    #[error("HTTP error: {}", status)]
    Http { status: http::StatusCode },
    #[error("could not parse data from JSON: {}", source)]
    DataType {
        #[from]
        source: serde_json::Error,
    },
    #[error("CDG Error: {}", source)]
    Cdg {
        #[from]
        source: CdgError,
    },
}
