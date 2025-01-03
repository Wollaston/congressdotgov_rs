use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /daily-congressional-record/:volumeNumber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct VolumeNumber {
    #[builder(setter(into))]
    volume_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl VolumeNumber {
    pub fn builder() -> VolumeNumberBuilder {
        VolumeNumberBuilder::default()
    }
}

impl Endpoint for VolumeNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("daily-congressional-record/{}", self.volume_number).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);
        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::Auth, cdg::Cdg, api::query::Query};

    use super::*;

    #[test]
    fn is_sufficient() {
        VolumeNumber::builder()
            .volume_number(166_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = VolumeNumber::builder()
            .volume_number(116_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
