//! A trait representing a client which can communicate with a GitLab instance via REST.

use bytes::Bytes;
use http::Response;
use std::error::Error;
use url::Url;

use crate::api::error::ApiError;

pub trait Client {
    type Error: Error + Send + Sync + 'static;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    fn set_auth(&self, url: &mut Url);

    fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> impl std::future::Future<Output = Result<Response<Bytes>, ApiError<Self::Error>>> + Send;
}
