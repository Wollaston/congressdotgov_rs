use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /senate-communication/:congress endpoint.
#[derive(Debug, Clone, Copy, Builder)]
#[builder(setter(strip_option))]
pub struct Congress {
    #[builder(default)]
    congress: u8,
    #[builder(default)]
    offset: Option<u32>,
    #[builder(default)]
    limit: Option<u8>,
}

impl Congress {
    pub fn builder() -> CongressBuilder {
        CongressBuilder::default()
    }
}

impl Endpoint for Congress {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("senate-communication/{}", self.congress).into()
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
        api::{common::Format, query::Query, senate_communication::congress::Congress},
        auth::Auth,
        cdg::Cdg,
    };

    #[test]
    fn is_sufficient() {
        Congress::builder().congress(117_u8).build().unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = Congress::builder().congress(117_u8).build().unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
