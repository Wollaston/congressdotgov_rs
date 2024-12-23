use bytes::Bytes;
use http::Response;
use std::error::Error;
use url::Url;

use crate::api::ApiError;

pub trait Client {
    type Error: Error + Send + Sync + 'static;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>>;

    fn set_auth(&self, url: &mut Url);

    async fn rest(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}
