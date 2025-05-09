use derive_builder::Builder;
use http::Method;
use std::borrow::Cow;

use crate::{api::endpoint::Endpoint, api::params::QueryParams};

/// Represents the /member/:bioguideId endpoint.
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct BioguideId<'a> {
    #[builder(setter(into))]
    bioguide_id: Cow<'a, str>,
}

impl<'a> BioguideId<'a> {
    pub fn builder() -> BioguideIdBuilder<'a> {
        BioguideIdBuilder::default()
    }
}

impl Endpoint for BioguideId<'_> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("member/{}", self.bioguide_id).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{api::common::Format, api::query::Query, auth::Auth, cdg::Cdg};

    use super::*;

    #[test]
    fn is_sufficient() {
        BioguideId::builder()
            .bioguide_id("L000174")
            .build()
            .unwrap();
    }

    #[tokio::test]
    async fn endpoint() {
        dotenvy::dotenv().unwrap();

        let auth = Auth::Token(dotenvy::var("CDG_API_KEY").unwrap());
        let req_client = reqwest::Client::new();
        let client = Cdg::new(auth, req_client, Format::Json).unwrap();

        let endpoint = BioguideId::builder()
            .bioguide_id("L000174")
            .build()
            .unwrap();

        let _res: serde_json::Value = endpoint.query(&client).await.unwrap();
    }
}
