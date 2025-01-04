use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /nomination/:congress/:nominationNumber endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct NominationNumber {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    nomination_number: u32,
    #[builder(default)]
    format: Format,
}

impl NominationNumber {
    pub fn builder() -> NominationNumberBuilder {
        NominationNumberBuilder::default()
    }
}

impl Endpoint for NominationNumber {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("nomination/{}/{}", self.congress, self.nomination_number).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push("format", self.format);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        NominationNumber::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let client = Cdg::new(auth).unwrap();

        let endpoint = NominationNumber::builder()
            .congress(117_u8)
            .nomination_number(2467_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
