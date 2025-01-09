use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::common::Format, api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /house-requirement/:requirementNumber/matching-communications endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct MatchingCommunications {
    #[builder(setter(into))]
    requirement_number: u32,
    #[builder(default)]
    format: Format,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl MatchingCommunications {
    pub fn builder() -> MatchingCommunicationsBuilder {
        MatchingCommunicationsBuilder::default()
    }
}

impl Endpoint for MatchingCommunications {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "house-requirement/{}/matching-communications",
            self.requirement_number
        )
        .into()
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
    use crate::{api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        MatchingCommunications::builder()
            .requirement_number(8070_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
let client = Cdg::new(auth, req_client).unwrap();

        let endpoint = MatchingCommunications::builder()
            .requirement_number(8070_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
