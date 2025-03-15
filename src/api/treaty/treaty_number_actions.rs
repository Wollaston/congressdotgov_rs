use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /treaty/:congress/:treatyNumber/:treatySuffix/actions endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct TreatyNumberActions {
    #[builder(setter(into))]
    congress: u8,
    #[builder(setter(into))]
    treaty_number: u32,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl TreatyNumberActions {
    pub fn builder() -> TreatyNumberActionsBuilder {
        TreatyNumberActionsBuilder::default()
    }
}

impl Endpoint for TreatyNumberActions {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("treaty/{}/{}/actions", self.congress, self.treaty_number).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("offset", self.offset);
        params.push_opt("limit", self.limit);

        params
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        api::{common::Format, query::Query},
        auth::Auth,
        cdg::Cdg,
    };

    use super::*;

    #[test]
    fn is_sufficient() {
        TreatyNumberActions::builder()
            .congress(117_u8)
            .treaty_number(3_u32)
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = TreatyNumberActions::builder()
            .congress(117_u8)
            .treaty_number(3_u32)
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
